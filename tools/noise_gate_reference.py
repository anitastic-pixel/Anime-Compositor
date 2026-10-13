"""D-642 (S-4 of D-638): the fixtures for Noise Gate, written before the code.

Writes test sounds as 32-bit floating-point WAV files in `Fixtures/noise_gate/media/`, one
project per case, and `expected_noise_gate.json`. The rule is document 21's (D-642), ours;
After Effects 26.0 gives the controls and their starting values, not the maths:
- an audio layer's sound effects run, in order, on its sound as the composition hears it
  (D-640), from the layer's first frame, before its level;
- Noise Gate keeps a gain g, 0 (shut) at the layer's first sample, and a hold count, 0. For
  each sample, with the louder channel's size p = max(|left|, |right|), the threshold
  T = 10^(threshold / 20) and the times in whole samples A = round(attack x 48),
  H = round(hold x 48), R = round(release x 48) (milliseconds, 48,000 samples a second,
  rounded half up):
    p >= T: hold = H, and g rises by 1/A (to 1 at once when A is 0);
    else, hold > 0: hold falls by 1, and g still rises as above;
    else: g falls by 1/R;
  g is kept between 0 and 1 (within 1e-12 of either, it is that end), and both channels
  are multiplied by g. So the gate opens in A samples, stays open H samples after the last
  sample at or above the threshold, and shuts R samples after that;
- a keyed number moves in a straight line across each frame's samples: at sample s of frame f
  it is v(f) + (v(f + 1) - v(f)) x (s - first(f)) / (first(f + 1) - first(f)), where v is the
  number at a composition frame (document 20, held inside its range) and
  first(f) = floor(f x 48000 x den / num);
- a Noise Gate switched off, or with a number outside its range (kept, with
  EFFECT_PARAMETER_INVALID), passes the sound unchanged.
The timing table gives, from the gain, for its first opening: the first sample where it is
above 0 (rise), the first where it reaches 1 (full), the first after full where it falls below
1 (closing), the first after that where it is 0 (shut), and the last sample before closing at
or above the threshold (last_open).
Each test sound sits on a steady floor at -70 dB, below every threshold, so the gain can be
read from the sound while nothing opens the gate.
Run from the repository root: python tools/noise_gate_reference.py
Needs numpy.
"""
import copy
import json
import math
import pathlib
import sys

import numpy as np

ROOT = pathlib.Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from sound_mix_reference import first_sample, frame_of, places, write_wav  # noqa: E402  D-640
from sound_read_reference import stereo, working  # noqa: E402  D-639's working sound

OUT = ROOT / "Fixtures" / "noise_gate"
MEDIA = OUT / "media"
WORK = 48000
FLOOR = 10 ** (-70 / 20)
DEFAULTS = {"threshold": -60.0, "attack": 10.0, "hold": 40.0, "release": 220.0}
RANGES = {"threshold": (-60, 0), "attack": (0, 1000), "hold": (0, 2500), "release": (5, 4000)}


def burst(n, at, length, db):
    k = np.arange(length)
    x = np.zeros(n)
    x[at:at + length] = 10 ** (db / 20) * np.cos(2 * np.pi * 1000 * k / WORK)
    return x


def media():
    MEDIA.mkdir(parents=True, exist_ok=True)
    s = {}
    n = int(1.5 * WORK)
    b = FLOOR + burst(n, 12000, 12000, -6)
    s["burst.wav"] = (write_wav(MEDIA / "burst.wav", np.stack([b, b], 1), WORK), WORK)
    n = 2 * WORK
    t = FLOOR + burst(n, 12000, 12000, -26) + burst(n, 48000, 12000, -6)
    s["two_bursts.wav"] = (write_wav(MEDIA / "two_bursts.wav", np.stack([t, t], 1), WORK), WORK)
    d = np.full(n, 0.04)
    s["steady.wav"] = (write_wav(MEDIA / "steady.wav", np.stack([d, d], 1), WORK), WORK)
    left = np.full(int(1.5 * WORK), FLOOR)
    right = burst(len(left), 12000, 12000, -6)
    s["right_burst.wav"] = (write_wav(MEDIA / "right_burst.wav", np.stack([left, right], 1), WORK), WORK)
    return s


def gate_json(g):
    """A gate's parameters as the project file holds them: a number, or {base, keyframes}."""
    p = {}
    for k, v in DEFAULTS.items():
        v = g.get(k, v)
        if isinstance(v, list):
            p[k] = {"base": v[0][1], "keyframes": [{"frame": f, "value": x, "interp": "linear"} for f, x in v]}
        else:
            p[k] = v
    return p


def project(case, layers, num, den, duration):
    assets, made = [], []
    for k, l in enumerate(layers):
        if l.get("solid"):
            made.append({"id": "solid%d" % k, "kind": "solid", "name": "solid %d" % k, "enabled": True,
                         "locked": False, "in_frame": 0, "out_frame": duration,
                         "transform": {"anchor": {"base": [0, 0], "keyframes": []},
                                       "position": {"base": [0, 0], "keyframes": []},
                                       "scale": {"base": [100, 100], "keyframes": []},
                                       "rotation": {"base": 0, "keyframes": []},
                                       "opacity": {"base": 1, "keyframes": []}},
                         "mask": None, "matte": None, "blend_mode": "normal",
                         "solid": {"color": [1.0, 1.0, 1.0], "width": 4, "height": 2},
                         "effects": l["effects_json"]})
            continue
        aid = "asset-%d" % k
        assets.append({"id": aid, "kind": "audio", "name": l["file"].split(".")[0], "path": "media/" + l["file"]})
        m = {"id": "sound%d" % k, "kind": "audio", "name": "sound %d" % k, "asset_id": aid,
             "enabled": True, "locked": False, "in_frame": l.get("in", 0),
             "out_frame": duration, "source_offset_frames": 0, "gain_db": l.get("gain", 0)}
        if "effects_json" in l:
            m["effects"] = l["effects_json"]
        else:
            m["effects"] = [{"instance_id": "gate-%d" % i, "type_id": "core.noise_gate",
                             "enabled": g.get("on", True), "parameters": gate_json(g)}
                            for i, g in enumerate(l.get("gates", []))]
        made.append(m)
    return {
        "schema_version": 0,
        "project_id": "proj-" + case,
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": assets,
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": 4, "height": 2, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": num, "denominator": den}, "start_frame": 0,
            "duration_frames": duration,
            "work_area": {"start_frame": 0, "end_frame_exclusive": duration},
            "layer_order": [m["id"] for m in made], "layers": made,
        }],
    }


def at_frame(v, f):
    """Document 20's linear value of a number, held before its first key and after its last."""
    if not isinstance(v, list):
        return float(v)
    if f <= v[0][0]:
        return float(v[0][1])
    for (f0, x0), (f1, x1) in zip(v, v[1:]):
        if f <= f1:
            return x0 + (x1 - x0) * ((f - f0) / (f1 - f0))
    return float(v[-1][1])


def number(g, name, s, num, den):
    v = g.get(name, DEFAULTS[name])
    if not isinstance(v, list):
        return float(v)
    lo, hi = RANGES[name]
    f = frame_of(s, num, den)
    s0, s1 = first_sample(f, num, den), first_sample(f + 1, num, den)
    a = min(max(at_frame(v, f), lo), hi)
    b = min(max(at_frame(v, f + 1), lo), hi)
    return a + (b - a) * (s - s0) / (s1 - s0)


def whole(ms):
    return int(math.floor(ms * 48 + 0.5))


def in_range(g):
    for k, (lo, hi) in RANGES.items():
        v = g.get(k, DEFAULTS[k])
        for x in ([y for _, y in v] if isinstance(v, list) else [v]):
            if not lo <= x <= hi:
                return False
    return True


def gate(x, s0, spec, num, den):
    """Noise Gate over x, whose first row is composition sample s0. Gives (out, gains, opened),
    opened saying which samples were at or above the threshold."""
    out = x.copy()
    gains = np.zeros(len(x))
    opened = np.zeros(len(x), dtype=bool)
    g, hold = 0.0, 0
    for i in range(len(x)):
        s = s0 + i
        t = 10 ** (number(spec, "threshold", s, num, den) / 20)
        a = whole(number(spec, "attack", s, num, den))
        h = whole(number(spec, "hold", s, num, den))
        r = whole(number(spec, "release", s, num, den))
        p = max(abs(x[i, 0]), abs(x[i, 1]))
        if p >= t:
            hold = h
            opened[i] = True
            g = 1.0 if a == 0 else g + 1.0 / a
        elif hold > 0:
            hold -= 1
            g = 1.0 if a == 0 else g + 1.0 / a
        else:
            g = g - 1.0 / r
        if g >= 1.0 - 1e-12:
            g = 1.0
        if g <= 1e-12:
            g = 0.0
        gains[i] = g
        out[i] = x[i] * g
    return out, gains, opened


def layer_sound(l, sounds, num, den, s_end):
    """The layer as the composition hears it from its first sample to s_end, before its level."""
    x, rate = sounds[l["file"]]
    w = working(stereo(x), rate)
    start = first_sample(l.get("in", 0), num, den)
    y = np.zeros((s_end - start, 2))
    for s in range(start, s_end):
        f = frame_of(s, num, den)
        if not (l.get("in", 0) <= f < l["out"]):
            continue
        n = f - l.get("in", 0)
        i = first_sample(n, num, den) + (s - first_sample(f, num, den))
        if 0 <= i < len(w):
            y[s - start] = w[i]
    return start, y


def events(gains, start, opened):
    """The first opening: rise, full, its last sample at or above the threshold, closing, shut."""
    def first(cond, frm=0):
        for i in range(frm, len(gains)):
            if cond(gains[i]):
                return i
        return None
    rise = first(lambda v: v > 0)
    full = first(lambda v: v == 1.0, rise) if rise is not None else None
    closing = first(lambda v: v < 1.0, full) if full is not None else None
    shut = first(lambda v: v == 0.0, closing) if closing is not None else None

    end = closing if closing is not None else len(gains)
    last = [i for i in range(end) if opened[i]]

    def at(i):
        return None if i is None else start + i
    return {"rise": at(rise), "full": at(full), "last_open": at(last[-1]) if last else None,
            "closing": at(closing), "shut": at(shut)}


def mix(layers, sounds, num, den, a, k):
    """The composition's mix of frames a .. a + k, and the last gate's timing."""
    s0, s1 = first_sample(a, num, den), first_sample(a + k, num, den)
    out = np.zeros((s1 - s0, 2))
    timing = None
    for l in layers:
        start, y = layer_sound(l, sounds, num, den, s1)
        for g in l.get("gates", []):
            if not g.get("on", True) or not in_range(g):
                continue
            y, gains, opened = gate(y, start, g, num, den)
            timing = events(gains, start, opened)
        gain = 10 ** (l.get("gain", 0) / 20)
        for s in range(max(s0, start), s1):
            out[s - s0] += gain * y[s - start]
    return s0, out, timing


def db(v):
    return None if v == 0 else round(20 * math.log10(v), 6)


KEYED_THRESHOLD = [[0, -40], [10, -20]]
KEYED_ATTACK = [[0, 0], [10, 100]]

# (case, what it shows, layers, (num, den), first frame, frames, samples to look around)
CASES = [
    ("fx_gate_001", "the starting values on a burst at sample 12000: opens over 480 samples, holds 1920, shuts over 10560",
     [{"file": "burst.wav", "gates": [{}]}], (24, 1), 0, 48, [12000, 12479, 24000, 25920, 36479]),
    ("fx_gate_002", "Attack 0: fully open on the burst's first sample",
     [{"file": "burst.wav", "gates": [{"attack": 0}]}], (24, 1), 0, 48, [12000, 25920]),
    ("fx_gate_003", "Hold 500 ms: open 24000 samples after the burst ends",
     [{"file": "burst.wav", "gates": [{"hold": 500}]}], (24, 1), 0, 48, [24000, 48000, 58559]),
    ("fx_gate_004", "Hold 0 and Release 5 ms: it dips at the burst's quiet zero crossings, and is shut 240 samples after the burst",
     [{"file": "burst.wav", "gates": [{"hold": 0, "release": 5}]}], (24, 1), 0, 48, [24000, 24239]),
    ("fx_gate_005", "Threshold -20 dB: the quiet burst (-26 dB) stays shut, the loud one (-6 dB) opens it",
     [{"file": "two_bursts.wav", "gates": [{"threshold": -20}]}], (24, 1), 0, 48, [12000, 48000, 48479]),
    ("fx_gate_006", "Threshold -60 dB: both bursts open it",
     [{"file": "two_bursts.wav", "gates": [{}]}], (24, 1), 0, 48, [12000, 48000]),
    ("fx_gate_007", "Threshold keyed from -40 dB (frame 0) to -20 dB (frame 10) over a steady -28 dB: it shuts part-way through frame 6",
     [{"file": "steady.wav", "gates": [{"threshold": KEYED_THRESHOLD}]}], (24, 1), 0, 24, [0, 479, 12040]),
    ("fx_gate_008", "the right channel's burst opens the gate for both: the quiet left is heard while it is open",
     [{"file": "right_burst.wav", "gates": [{}]}], (24, 1), 0, 48, [12000, 25920]),
    ("fx_gate_009", "frames 10 to 19 of fx_gate_001 alone: the same samples as the whole mix there",
     [{"file": "burst.wav", "gates": [{}]}], (24, 1), 10, 10, [25920, 36479]),
    ("fx_gate_010", "a layer starting at frame 6 with a level of -6 dB: the gate counts from the layer's start, before the level",
     [{"file": "burst.wav", "in": 6, "gain": -6, "gates": [{"threshold": -10}]}], (24, 1), 0, 48, [24000, 24479]),
    ("fx_gate_011", "the gate switched off: the sound unchanged",
     [{"file": "burst.wav", "gates": [{"on": False}]}], (24, 1), 0, 48, [12000]),
    ("fx_gate_012", "Attack keyed from 0 ms (frame 0) to 100 ms (frame 10) at 29.97: the ramp lengthens sample by sample",
     [{"file": "burst.wav", "gates": [{"attack": KEYED_ATTACK}]}], (30000, 1001), 0, 48, [12000, 12812]),
    ("fx_gate_013", "two gates in a row: the second opens on what the first lets through",
     [{"file": "two_bursts.wav", "gates": [{}, {"threshold": -20, "attack": 0}]}], (24, 1), 0, 48, [12000, 48000]),
    ("fx_gate_014", "a Threshold of +3 dB, outside -60 to 0: kept, EFFECT_PARAMETER_INVALID, the sound unchanged",
     [{"file": "burst.wav", "gates": [{"threshold": 3}]}], (24, 1), 0, 24, [12000]),
]

# Files that must not open: (case, what, layers)
REFUSED = [
    ("fx_gate_101", "a picture effect (Exposure) on an audio layer is PROJECT_SCHEMA_INVALID",
     [{"file": "burst.wav", "effects_json": [{"instance_id": "fx-0", "type_id": "core.exposure", "enabled": True,
                                              "parameters": {"stops": 1}}]}]),
    ("fx_gate_102", "Noise Gate on a solid layer is PROJECT_SCHEMA_INVALID",
     [{"solid": True, "effects_json": [{"instance_id": "gate-0", "type_id": "core.noise_gate", "enabled": True,
                                        "parameters": gate_json({})}]}]),
    ("fx_gate_103", "an expression on a Noise Gate number is PROJECT_SCHEMA_INVALID: sound effects take keys only",
     [{"file": "burst.wav", "effects_json": [{"instance_id": "gate-0", "type_id": "core.noise_gate", "enabled": True,
                                              "parameters": dict(gate_json({}), threshold={
                                                  "base": -60, "keyframes": [], "expression": "-30"})}]}]),
    ("fx_gate_104", "a Mix on Noise Gate is PROJECT_SCHEMA_INVALID: a sound effect has no Mix",
     [{"file": "burst.wav", "effects_json": [{"instance_id": "gate-0", "type_id": "core.noise_gate", "enabled": True,
                                              "parameters": gate_json({}), "mix": 50}]}]),
]

# An effect this build does not know, on an audio layer: kept, EFFECT_UNSUPPORTED, the sound unchanged.
UNKNOWN = ("fx_gate_201", "an effect this build does not know on an audio layer: kept and saved as written, "
           "EFFECT_UNSUPPORTED, the sound unchanged",
           [{"file": "burst.wav", "effects_json": [{"instance_id": "fx-0", "type_id": "vendor.reverb", "enabled": True,
                                                    "parameters": {"room": 0.5}}]}])


def main():
    sounds = media()
    out = {"_source": "tools/noise_gate_reference.py (D-642)", "tolerance": 2e-5,
           "defaults": DEFAULTS, "ranges": RANGES, "cases": [], "refused": [], "unknown": None}
    duration = 48
    for case, what, layers, (num, den), a, k, extra in CASES:
        for l in layers:
            l.setdefault("out", duration)
        doc = project(case, layers, num, den, duration)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        s0, m, timing = mix(copy.deepcopy(layers), sounds, num, den, a, k)
        at = places(s0, len(m), extra)
        peak = np.abs(m).max(axis=0)
        bad = not all(in_range(g) for l in layers for g in l["gates"])
        out["cases"].append({
            "project": case + ".json", "what": what, "from": a, "frames": k,
            "first_sample": s0, "samples": len(m),
            "peak_db": [db(float(peak[0])), db(float(peak[1]))],
            "timing": timing, "warnings": ["EFFECT_PARAMETER_INVALID"] if bad else [], "places": at,
            "values": [[float(v) for v in m[i]] for i in at],
        })
    for case, what, layers in REFUSED:
        doc = project(case, layers, 24, 1, duration)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        out["refused"].append({"project": case + ".json", "what": what, "diagnostic": "PROJECT_SCHEMA_INVALID"})
    case, what, layers = UNKNOWN
    for l in layers:
        l.setdefault("out", duration)
    doc = project(case, layers, 24, 1, duration)
    (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
    s0, m, _ = mix([dict(layers[0], gates=[])], sounds, 24, 1, 0, 24)
    at = places(s0, len(m), [12000])
    out["unknown"] = {"project": case + ".json", "what": what, "warnings": ["EFFECT_UNSUPPORTED"],
                      "from": 0, "frames": 24, "places": at, "values": [[float(v) for v in m[i]] for i in at]}
    (OUT / "expected_noise_gate.json").write_text(json.dumps(out, indent=1) + "\n", encoding="utf-8")
    print("wrote", len(CASES) + len(REFUSED) + 1, "cases")
    for c in out["cases"]:
        print(c["project"], c["timing"], c["peak_db"])


if __name__ == "__main__":
    main()
