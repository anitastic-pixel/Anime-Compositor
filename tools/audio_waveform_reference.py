"""Audio Waveform, worked a second way.

D-421 adds `core.audio_waveform`, After Effects' Audio Waveform (Generate): it "display[s] the
audio waveform of a layer that contains audio", along a line or "along an open or closed mask
path" (Adobe's After Effects user manual, pages 510 to 511 of the copy on manualsdir.com). Its
settings are After Effects': Audio Layer, Start Point, End Point, Path, Displayed Samples,
Maximum Height, Audio Duration, Audio Offset, Thickness, Softness, Random Seed (Analog), Inside
Color, Outside Color, Waveform Options, Display Options and Composite On Original.

What Adobe says, and this program follows: Waveform Options "Mono combines the left and right
channels of the audio layer. Nonstereo audio layers play as Mono." Display Options "Digital
displays each sample as a single vertical line connecting the minimum and maximum source
sample"; "Analog Lines displays each sample as a line connecting the previous and next sample
from either the minimum or maximum audio source sample"; "Analog Dots displays each sample as a
dot representing either the minimum or maximum audio source sample". Composite On Original
"composites the audio waveform with the original layer using the Add blending mode. When
deselected, only the audio waveform is visible." Adobe does not say how the displayed samples
are taken from the source, how tall a sample stands, nor how the random seed picks minimum or
maximum; the rule below is this program's own, nothing is ported.

The sound is Audio Spectrum's (D-420, `tools/audio_spectrum_reference.py`): the Audio Layer by
its id ("" none: silence), the window's first sample s0 at composition frame f
(floor(n rate den / num) + floor(offset rate / 1000 + 1/2), n the layer's own frame, silence
outside the layer), N = max(1, floor(duration rate / 1000 + 1/2)) samples, a sample times the
layer's gain 10^(gain/20) and 0 before the file's first sample or past its last. Waveform
Options mono takes the channels' mean, left channel 0, right channel 1 (channel 0 of a file
with one channel).

The displayed samples. D = floor(Displayed Samples); displayed sample j (0 <= j < D) covers the
source samples from s0 + floor(j N / D) up to, not including, s0 + floor((j + 1) N / D), or the
first alone when that is none; lo_j and hi_j are the least and greatest of them, each held to
-1..1. At silence both are 0.

Where they stand, t_j = (j + 1/2) / D along the way, in the drawing's own pixels: on the line
from Start Point to End Point (per cent of the drawing), facing n = (dy, -dx) for the line's
direction (dx, dy), up for a line drawn left to right ((0, -1) when the two points meet); or,
Path k, along mask k of the layer as document 19 flattens it (closed), at t_j of its length,
facing the normal of its piece. A mask that is not there, off, or of fewer than two points:
nothing is drawn and EFFECT_PATH_MISSING is said. A level v stands at p + Maximum Height v n.

The marks, each a straight piece drawn Thickness across:
- Digital: for each displayed sample, a piece from its foot + Maximum Height lo_j n to its foot
  + Maximum Height hi_j n, each its own mark.
- Analog Lines and Analog Dots take v_j = hi_j when the top bit of
  mix(mix(floor(Random Seed)) xor j) is 1, else lo_j, mix SplitMix64's finaliser (D-119's,
  `grade::mix`). Analog Lines: one mark through the points, from point j to point j + 1, and
  from the last to the first when the way is closed (a mask); a dot when D is 1. Analog Dots: a
  dot (a piece of no length) at each point, each its own mark.

Drawing is Audio Spectrum's with one colour pair (Inside Color, Outside Color, as written) and
Blend Overlapping Colors off: a mark's distance d is the least to its pieces; Beam's covering c
and colour L (D-207, `line_profile`); the marks laid one over the next. Composite On Original on:
added to the layer, colour P + L c, alpha min(P_a + c, 1); off: (L c, c) alone. The layer never
grows.

`audio_layer`, a layer id or ""; `start_point` and `end_point` -1000 to 1000 per cent, keyable,
(10, 50) and (90, 50) when added; `path` 0 to 1000, 0; `displayed_samples` 1 to 4096, keyable,
32; `maximum_height` 0 to 10000 pixels, keyable, 300; `audio_duration` 1 to 30000 milliseconds,
keyable, 90; `audio_offset` -30000 to 30000 milliseconds, keyable, 0; `thickness` 0 to 10000
pixels, keyable, 2; `softness` 0 to 100, keyable, 50; `random_seed` 0 to 100000, keyable, its
whole part counted, 1; `inside_color` white and `outside_color` #3c8cff, read in small letters;
`waveform_options` "mono", "left" or "right", mono; `display_options` "digital",
"analog_lines" or "analog_dots", analog_lines; `composite` off. A draft halves Maximum Height and
Thickness, which are distances.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values and from the samples Audio Spectrum's reference writes.

Every case is a composition 16 by 10 at 24 frames a second holding Gradient's cel, the same
size, unmoved unless the case says, and a sound layer "sound" over all five frames, playing one
of Audio Spectrum's sounds, written again here into `Fixtures/audio_waveform/media/`. The
expected frames are in `Fixtures/audio_waveform/expected_audio_waveform.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/audio_waveform_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import audio_spectrum_reference as AS  # noqa: E402
import checkerboard_reference as K  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from fxkey_reference import keyed, value_at  # noqa: E402
from mask_reference import flatten  # noqa: E402

W, H = K.W, K.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "audio_waveform"
TOLERANCE = 2e-5  # document 25's default for a filter
RATE = AS.RATE
MASK = (1 << 64) - 1
RANGES = {"start_point": (-1000, 1000), "end_point": (-1000, 1000), "path": (0, 1000),
          "displayed_samples": (1, 4096), "maximum_height": (0, 10000),
          "audio_duration": (1, 30000), "audio_offset": (-30000, 30000),
          "thickness": (0, 10000), "softness": (0, 100), "random_seed": (0, 100000)}
WORDS = ("audio_layer", "inside_color", "outside_color", "waveform_options", "display_options",
         "composite")
NAMES = ("audio_layer", "start_point", "end_point", "path", "displayed_samples",
         "maximum_height", "audio_duration", "audio_offset", "thickness", "softness",
         "random_seed", "inside_color", "outside_color", "waveform_options", "display_options",
         "composite")
VIOLET, BLUE, ORANGE = K.VIOLET, "#3c8cff", "#ff8000"


def mix(z):
    """SplitMix64's finaliser, D-119's `grade::mix`."""
    z = (z + 0x9E3779B97F4A7C15) & MASK
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
    return z ^ (z >> 31)


def takes_high(seed, j):
    return mix(mix(int(math.floor(seed))) ^ j) >> 63 == 1


def spans(snd, frame_no, n, channel):
    """(lo_j, hi_j) for every displayed sample."""
    shown = int(math.floor(n["displayed_samples"]))
    s0 = None if snd is None else AS.window_start(snd, frame_no, n["audio_offset"])
    if s0 is None:
        return [(0.0, 0.0)] * shown
    count = max(1, math.floor(n["audio_duration"] * RATE / 1000 + 0.5))
    gain = 10 ** (snd["gain"] / 20)
    out = []
    for j in range(shown):
        a, b = s0 + j * count // shown, s0 + (j + 1) * count // shown
        vs = [min(1.0, max(-1.0, AS.sample(snd, i, gain, channel))) for i in range(a, max(b, a + 1))]
        out.append((min(vs), max(vs)))
    return out


def feet(n, shown, outline):
    t = [(j + 0.5) / shown for j in range(shown)]
    if outline is not None:
        return [AS.along(outline, u) for u in t], True
    p, q = AS.per_cent(n["start_point"]), AS.per_cent(n["end_point"])
    normal = AS.facing(q[0] - p[0], q[1] - p[1])
    return [((p[0] + u * (q[0] - p[0]), p[1] + u * (q[1] - p[1])), normal) for u in t], False


def marks(spots, closed, s, height, cols, display, seed):
    at = lambda j, v: (spots[j][0][0] + height * v * spots[j][1][0],  # noqa: E731
                       spots[j][0][1] + height * v * spots[j][1][1])
    shown = len(spots)
    if display == "digital":
        return [[(at(j, s[j][0]), at(j, s[j][1])) + cols] for j in range(shown)]
    v = [s[j][1] if takes_high(seed, j) else s[j][0] for j in range(shown)]
    if display == "analog_dots":
        return [[(at(j, v[j]), at(j, v[j])) + cols] for j in range(shown)]
    if shown == 1:
        return [[(at(0, v[0]), at(0, v[0])) + cols]]
    pieces = [(at(j, v[j]), at(j + 1, v[j + 1])) + cols for j in range(shown - 1)]
    if closed:
        pieces.append((at(shown - 1, v[shown - 1]), at(0, v[0])) + cols)
    return [pieces]


# --- the cases ------------------------------------------------------------------------------

def case(audio_layer="", start_point=(10, 50), end_point=(90, 50), path=0, displayed_samples=32,
         maximum_height=300, audio_duration=90, audio_offset=0, thickness=2, softness=50,
         random_seed=1, inside_color="#ffffff", outside_color=BLUE, waveform_options="mono",
         display_options="analog_lines", composite="off", snd=None, masks=(), shift=0,
         tile=False):
    c = {k: v for k, v in locals().items() if k in NAMES}
    c.update({"drawing": "cel", "snd": snd or AS.sound(), "masks": list(masks), "shift": shift,
              "tile": tile})
    return c


def wave(**kw):
    """The small frame's own waveform: 8 displayed samples of a 10 ms window (48 samples, 6
    each) on a line across the middle, 4 pixels each way at most, 2 thick and sharp, alone."""
    base = dict(audio_layer="sound", displayed_samples=8, maximum_height=4, audio_duration=10,
                thickness=2, softness=0)
    base.update(kw)
    return case(**base)


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
    k = int(math.floor(n["path"]))
    if k == 0:
        return None
    if k > len(c["masks"]):
        return "missing"
    return flatten(c["masks"][k - 1]["points"])


def render(c, frame_no):
    n = settings(c, frame_no)
    layer = K.layer_of(c)
    outline = outline_of(c, n)
    if outline == "missing" or c["audio_layer"] not in ("", "sound"):
        return frame(layer, c["shift"])
    channel = {"mono": None, "left": 0, "right": 1}[c["waveform_options"]]
    s = spans(c["snd"] if c["audio_layer"] == "sound" else None, frame_no, n, channel)
    spots, closed = feet(n, len(s), outline)
    cols = (AS.turned(c["inside_color"], 0), AS.turned(c["outside_color"], 0))
    ms = marks(spots, closed, s, n["maximum_height"], cols, c["display_options"],
               n["random_seed"])
    how = "add" if c["composite"] == "on" else "alone"
    return frame(AS.laid(layer, ms, n["thickness"], n["softness"], False, how), c["shift"])


def plain(c):
    return frame(K.layer_of(c), c["shift"])


BOX = AS.BOX

CASES = {
    "FX-AWAVE-001": ("The settings as they start, no Audio Layer: silence, so a flat Analog Line "
                     "2 thick along the line through the middle, alone.", case(), [0]),
    "FX-AWAVE-002": ("Eight displayed samples of a 10 ms window, 4 tall each way at most, Analog "
                     "Lines, 2 thick, sharp, alone: a zigzag across the middle, each point the "
                     "least or greatest of its six source samples as Random Seed 1 picks.",
                     wave(), [0, 2]),
    "FX-AWAVE-003": ("FX-AWAVE-002 with Composite On Original on: added to the cel.",
                     wave(composite="on"), [0]),
    "FX-AWAVE-004": ("Digital: each displayed sample a stroke from its least to its greatest "
                     "source sample.", wave(display_options="digital"), [0]),
    "FX-AWAVE-005": ("Analog Dots: a dot at each picked point.",
                     wave(display_options="analog_dots"), [0]),
    "FX-AWAVE-006": ("Random Seed 2: other picks of least or greatest.", wave(random_seed=2),
                     [0]),
    "FX-AWAVE-007": ("Random Seed 2.9: its whole part counted, so FX-AWAVE-006's frame.",
                     wave(random_seed=2.9), [0]),
    "FX-AWAVE-008": ("The stereo file, Waveform Options Left: the 600 Hz channel alone.",
                     wave(snd=AS.sound("stereo"), waveform_options="left"), [0]),
    "FX-AWAVE-009": ("The stereo file, Right: the 900 Hz channel alone.",
                     wave(snd=AS.sound("stereo"), waveform_options="right"), [0]),
    "FX-AWAVE-010": ("The stereo file, Mono: the two channels' mean.",
                     wave(snd=AS.sound("stereo")), [0]),
    "FX-AWAVE-011": ("The one-channel file with Right: it plays as Mono, so FX-AWAVE-002's "
                     "frame.", wave(waveform_options="right"), [0]),
    "FX-AWAVE-012": ("Audio Offset 5 ms: the window starts 24 samples later.",
                     wave(audio_offset=5), [0]),
    "FX-AWAVE-013": ("Audio Duration 40 ms: 192 samples, 24 to each displayed sample, so every "
                     "least and greatest is near the tone's own.", wave(audio_duration=40), [0]),
    "FX-AWAVE-014": ("Displayed Samples 1: one point in the middle of the line, a dot.",
                     wave(displayed_samples=1), [0]),
    "FX-AWAVE-015": ("Displayed Samples 48: one source sample each, the wave itself.",
                     wave(displayed_samples=48), [0]),
    "FX-AWAVE-016": ("Digital with Displayed Samples 60, more than the window's 48: some "
                     "displayed samples share a source sample.",
                     wave(display_options="digital", displayed_samples=60), [0]),
    "FX-AWAVE-017": ("Path 1, a mask (mode None) round the box from (2, 2) to (14, 8), 16 "
                     "displayed samples, 2 tall at most: a closed line round the box.",
                     wave(path=1, masks=[{"points": BOX, "mode": "none"}], displayed_samples=16,
                          maximum_height=2), [0]),
    "FX-AWAVE-018": ("Path 2 with only one mask: none to draw along, so nothing is drawn and "
                     "EFFECT_PATH_MISSING is said; the cel as it was.",
                     wave(path=2, masks=[{"points": BOX, "mode": "none"}], composite="on"), [0]),
    "FX-AWAVE-019": ("Audio Layer \"ghost\", not in the composition: EFFECT_LAYER_MISSING is "
                     "said and the cel is as it was.", wave(audio_layer="ghost", composite="on"),
                     [0]),
    "FX-AWAVE-020": ("Audio Layer \"art\", the cel itself, which holds no sound: "
                     "EFFECT_SOUND_MISSING is said and the cel is as it was.",
                     wave(audio_layer="art", composite="on"), [0]),
    "FX-AWAVE-021": ("The 24-bit file: as FX-AWAVE-002 within a level.",
                     wave(snd=AS.sound("tone24")), [0]),
    "FX-AWAVE-022": ("The 32-bit floating point file.", wave(snd=AS.sound("tone_float")), [0]),
    "FX-AWAVE-023": ("The 8-bit file.", wave(snd=AS.sound("tone8")), [0]),
    "FX-AWAVE-024": ("The sound layer starting at frame 2: frames 0 and 1 silent (a flat line), "
                     "frame 4 its own frame 2.", wave(snd=AS.sound(in_=2)), [0, 4]),
    "FX-AWAVE-025": ("The sound layer at -6.0206 dB, half as loud: the wave half as tall.",
                     wave(snd=AS.sound(gain=-6.020599913279624)), [0]),
    "FX-AWAVE-026": ("The sound layer at +12 dB: four times as loud, held to Maximum Height.",
                     wave(snd=AS.sound(gain=12.0)), [0]),
    "FX-AWAVE-027": ("Maximum Height keyed from 0 at frame 0 to 4 at frame 4, linear.",
                     wave(maximum_height=keyed((0, 0), (4, 4))), [0, 2, 4]),
    "FX-AWAVE-028": ("FX-AWAVE-002 moved three pixels right: the wave moves with the layer.",
                     wave(shift=3), [0]),
    "FX-AWAVE-029": ("After a Motion Tile that grows the layer: the points are the drawing's "
                     "own, so the frame is FX-AWAVE-002's.", wave(tile=True), [0]),
    "FX-AWAVE-030": ("Thickness 0: nothing drawn, the cel as it was.",
                     wave(thickness=0, composite="on"), [0]),
    "FX-AWAVE-031": ("Thickness 3, softness 100, orange inside, violet outside.",
                     wave(thickness=3, softness=100, inside_color=ORANGE, outside_color=VIOLET),
                     [0]),
    "FX-AWAVE-032": ("Start Point (90, 50), End Point (10, 50): drawn right to left, so the "
                     "wave faces down.", wave(start_point=(90, 50), end_point=(10, 50)), [0]),
    "FX-AWAVE-033": ("A slant from (10, 20) to (90, 80).",
                     wave(start_point=(10, 20), end_point=(90, 80)), [0]),
}

INVALID = {
    "FX-AWAVE-034": ("Displayed Samples 0, below 1.", case(displayed_samples=0)),
    "FX-AWAVE-035": ("Displayed Samples 4097, above 4096.", case(displayed_samples=4097)),
    "FX-AWAVE-036": ("Maximum Height -1, below 0.", case(maximum_height=-1)),
    "FX-AWAVE-037": ("Audio Duration 0, below 1.", case(audio_duration=0)),
    "FX-AWAVE-038": ("Audio Offset -30001, below -30000.", case(audio_offset=-30001)),
    "FX-AWAVE-039": ("Thickness -1, below 0.", case(thickness=-1)),
    "FX-AWAVE-040": ("Softness 101, above 100.", case(softness=101)),
    "FX-AWAVE-041": ("Random Seed -1, below 0.", case(random_seed=-1)),
    "FX-AWAVE-042": ("Path 1001, above 1000.", case(path=1001)),
    "FX-AWAVE-043": ("End Point -1001 per cent down, below -1000.", case(end_point=(90, -1001))),
    "FX-AWAVE-044": ("Waveform Options \"stereo\", not mono, left or right.",
                     case(waveform_options="stereo")),
    "FX-AWAVE-045": ("Display Options \"bars\", not digital, analog_lines or analog_dots.",
                     case(display_options="bars")),
    "FX-AWAVE-046": ("Composite \"yes\", not on or off.", case(composite="yes")),
    "FX-AWAVE-047": ("Outside colour \"blue\", not #rrggbb.", case(outside_color="blue")),
    "FX-AWAVE-048": ("Audio Layer 5, a number, not a layer's name.", case(audio_layer=5)),
    "FX-AWAVE-049": ("Displayed Samples keyed to 4097 at frame 4, above 4096.",
                     case(displayed_samples=keyed((0, 32), (4, 4097)))),
}

WARNINGS = {"FX-AWAVE-018": "EFFECT_PATH_MISSING", "FX-AWAVE-019": "EFFECT_LAYER_MISSING",
            "FX-AWAVE-020": "EFFECT_SOUND_MISSING"}


def write(fx, c):
    return AS.write(fx, c, out=OUT, type_id="core.audio_waveform", names=NAMES, words=WORDS)


def main():
    AS.media(OUT)
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
    (OUT / "expected_audio_waveform.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    clear = [0.0, 0.0, 0.0, 0.0]
    art = plain(case())
    flat = c["FX-AWAVE-001"]["0"]
    assert flat[at(8, 5)][3] > 0.9 and near(flat[at(8, 0)], clear) and near(flat[at(8, 7)], clear)
    two = c["FX-AWAVE-002"]["0"]
    n = settings(wave(), 0)
    s = spans(AS.sound(), 0, n, None)
    assert all(-0.72 < lo < hi < 0.72 for lo, hi in s) and max(hi for _, hi in s) > 0.7, s
    assert len({takes_high(1, j) for j in range(8)}) == 2  # the seed picks both ways
    assert two != flat and any(two[at(x, y)][3] > 0.5 for x in range(W) for y in (2, 3))
    add = c["FX-AWAVE-003"]["0"]
    assert all(near(add[i], [art[i][k] + two[i][k] for k in range(3)]
                    + [min(art[i][3] + two[i][3], 1.0)]) for i in range(W * H))
    assert len({json.dumps(c[fx]["0"]) for fx in ("FX-AWAVE-002", "FX-AWAVE-004",
                                                   "FX-AWAVE-005", "FX-AWAVE-006")}) == 4
    assert c["FX-AWAVE-007"]["0"] == c["FX-AWAVE-006"]["0"]
    assert len({json.dumps(c[fx]["0"]) for fx in ("FX-AWAVE-008", "FX-AWAVE-009",
                                                   "FX-AWAVE-010")}) == 3
    left = spans(AS.sound("stereo"), 0, n, 0)
    assert max(hi for _, hi in left) > 0.55
    assert c["FX-AWAVE-011"]["0"] == two
    assert c["FX-AWAVE-012"]["0"] != two and c["FX-AWAVE-013"]["0"] != two
    one = c["FX-AWAVE-014"]["0"]
    assert near(one[at(1, 5)], clear) and any(one[at(8, y)][3] > 0.3 for y in range(H))
    box = c["FX-AWAVE-017"]["0"]
    assert near(box[at(8, 5)], clear) and any(box[at(x, 2)][3] > 0 for x in range(W))
    for fx in WARNINGS:
        assert c[fx]["0"] == art
    assert all(near(p, q, 4e-3) for p, q in zip(c["FX-AWAVE-021"]["0"], two))
    assert all(near(p, q, 4e-3) for p, q in zip(c["FX-AWAVE-022"]["0"], two))
    late = c["FX-AWAVE-024"]
    assert late["0"] == flat_wave() and late["4"] == c["FX-AWAVE-002"]["2"]
    half = spans(AS.sound(gain=-6.020599913279624), 0, n, None)
    assert all(abs(h[0] - v[0] / 2) < 1e-9 and abs(h[1] - v[1] / 2) < 1e-9
               for h, v in zip(half, s))
    loud = spans(AS.sound(gain=12.0), 0, n, None)
    assert max(hi for _, hi in loud) == 1.0 and min(lo for lo, _ in loud) == -1.0
    grow = c["FX-AWAVE-027"]
    assert grow["0"] == flat_wave() and grow["0"] != grow["2"]
    moved = c["FX-AWAVE-028"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-AWAVE-029"]["0"] == two
    assert c["FX-AWAVE-030"]["0"] == art
    down = c["FX-AWAVE-032"]["0"]
    assert down != two
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


def flat_wave():
    """FX-AWAVE-002's settings at silence: a flat line."""
    c = wave()
    c["audio_layer"] = ""
    return render(c, 0)


if __name__ == "__main__":
    if sys.argv[1:] == ["--check"]:
        check(json.loads((OUT / "expected_audio_waveform.json").read_text(encoding="utf-8")))
    else:
        main()
