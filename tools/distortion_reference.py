"""D-644 (S-6 of D-638): the fixtures for Distortion, written before the code.

Writes test sounds as 32-bit floating-point WAV files in `Fixtures/distortion/media/`, one
project per case, and `expected_distortion.json`. The rule is document 21's (D-644), ours;
After Effects 26.0 gives the controls and their starting values, not the maths:
- an audio layer's sound effects run, in order, on its sound from the layer's first frame,
  before its level (D-642);
- Distortion works on each channel's samples on their own. With x a sample, the pre-gain
  k = 10^(drive / 50) (Drive 0 to 100% is 0 to +40 dB) and u = k x, the type's curve f gives
  Soft Clip       f(u) = tanh(u)
  Hard Clip       f(u) = u held to -1 .. 1
  Saturation 1    f(u) = u / (1 + |u|)
  Saturation 2    f(u) = (2 / pi) atan(pi u / 2)
  Tube            f(u) = (tanh(u + 1/4) - tanh(1/4)) / (1 - tanh(1/4)^2)   (lopsided, so it adds
                  even harmonics; its slope at 0 is 1)
  Fuzz            f(u) = sign(u) min(1, sqrt(|u|))
  then Resolution b = floor(resolution) bits: below 24, w = floor(w q + 1/2) / q with
  q = 2^(b - 1); at 24 the sample is left as it is; then Downsample n = floor(downsample): a
  count, 0 at the layer's first sample, takes the sample when it is 0 and holds it for n
  samples (both channels together); 1 holds nothing. The wet sample is that times gain / 25
  (25 leaves it as the curve made it); out = volume / 11 x ((1 - m) x + m wet), m = wet share
  (Mix) / 100 (Volume 11 leaves it as it is);
- a keyed number moves in a straight line across each frame's samples (as Noise Gate's);
  Resolution and Downsample take the whole part of that;
- a Distortion switched off, or with a number outside its range or a type it does not have
  (kept, with EFFECT_PARAMETER_INVALID), passes the sound unchanged.
The harmonic table gives, for a 1 kHz sine at -6 dB through each type at the starting values,
the 2nd and 3rd harmonics' levels against the 1 kHz one, in dB, from samples 4,800 to 23,999
(400 whole cycles); a level below -100 dB is written as null ("none").
The curve table gives each type's output on the ramp at 21 named samples.
Run from the repository root: python tools/distortion_reference.py
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
from noise_gate_reference import at_frame, layer_sound, project  # noqa: E402  D-642

OUT = ROOT / "Fixtures" / "distortion"
MEDIA = OUT / "media"
WORK = 48000
TYPES = ["soft_clip", "hard_clip", "saturation_1", "saturation_2", "tube", "fuzz"]
DEFAULTS = {"type": "soft_clip", "drive": 25.0, "gain": 25.0, "wet": 100.0, "volume": 11.0,
            "resolution": 24.0, "downsample": 1.0}
RANGES = {"drive": (0, 100), "gain": (0, 300), "wet": (0, 100), "volume": (0, 100),
          "resolution": (1, 24), "downsample": (1, 32)}
NUMBERS = ["drive", "gain", "wet", "volume", "resolution", "downsample"]
T4 = math.tanh(0.25)
RAMP_AT = 24000
RAMP_LEN = 48000


def curve(kind, u):
    if kind == "soft_clip":
        return math.tanh(u)
    if kind == "hard_clip":
        return min(max(u, -1.0), 1.0)
    if kind == "saturation_1":
        return u / (1 + abs(u))
    if kind == "saturation_2":
        return 2 / math.pi * math.atan(math.pi * u / 2)
    if kind == "tube":
        return (math.tanh(u + 0.25) - T4) / (1 - T4 * T4)
    if kind == "fuzz":
        return math.copysign(min(1.0, math.sqrt(abs(u))), u)
    raise ValueError(kind)


def media():
    MEDIA.mkdir(parents=True, exist_ok=True)
    s = {}
    n = 2 * WORK
    k = np.arange(n)
    x = np.zeros(n)
    x[:RAMP_AT] = 10 ** (-6 / 20) * np.cos(2 * np.pi * 1000 * k[:RAMP_AT] / WORK)
    x[RAMP_AT:RAMP_AT + RAMP_LEN] = -1 + 2 * np.arange(RAMP_LEN) / (RAMP_LEN - 1)
    s["probe.wav"] = (write_wav(MEDIA / "probe.wav", np.stack([x, x], 1), WORK), WORK)
    return s


def params_json(spec):
    p = {}
    for k, v in DEFAULTS.items():
        v = spec.get(k, v)
        if isinstance(v, list):
            p[k] = {"base": v[0][1], "keyframes": [{"frame": f, "value": x, "interp": "linear"} for f, x in v]}
        else:
            p[k] = v
    return p


def effects_json(fx):
    return [{"instance_id": "dist-%d" % i, "type_id": "core.distortion", "enabled": spec.get("on", True),
             "parameters": params_json(spec)} for i, spec in enumerate(fx)]


def number(spec, name, s, num, den):
    v = spec.get(name, DEFAULTS[name])
    if not isinstance(v, list):
        return float(v)
    lo, hi = RANGES[name]
    f = frame_of(s, num, den)
    s0, s1 = first_sample(f, num, den), first_sample(f + 1, num, den)
    a = min(max(at_frame(v, f), lo), hi)
    b = min(max(at_frame(v, f + 1), lo), hi)
    return a + (b - a) * (s - s0) / (s1 - s0)


def in_range(spec):
    if spec.get("type", DEFAULTS["type"]) not in TYPES:
        return False
    for k, (lo, hi) in RANGES.items():
        v = spec.get(k, DEFAULTS[k])
        for x in ([y for _, y in v] if isinstance(v, list) else [v]):
            if not lo <= x <= hi:
                return False
    return True


def distort(x, s0, spec, num, den):
    out = x.copy()
    kind = spec.get("type", DEFAULTS["type"])
    keyed = any(isinstance(spec.get(k), list) for k in NUMBERS)
    fixed = {k: float(spec.get(k, DEFAULTS[k])) for k in NUMBERS} if not keyed else None
    count, held = 0, [0.0, 0.0]
    for i in range(len(x)):
        s = s0 + i
        v = fixed if fixed is not None else {k: number(spec, k, s, num, den) for k in NUMBERS}
        k = 10 ** (v["drive"] / 50)
        bits = int(math.floor(v["resolution"]))
        n = int(math.floor(v["downsample"]))
        w = [curve(kind, k * x[i, c]) for c in range(2)]
        if bits < 24:
            q = 2.0 ** (bits - 1)
            w = [math.floor(y * q + 0.5) / q for y in w]
        if count <= 0:
            held = w
            count = n
        count -= 1
        m = v["wet"] / 100
        for c in range(2):
            wet = held[c] * (v["gain"] / 25)
            out[i, c] = v["volume"] / 11 * ((1 - m) * x[i, c] + m * wet)
    return out


def mix(layers, sounds, num, den, a, k):
    s0, s1 = first_sample(a, num, den), first_sample(a + k, num, den)
    out = np.zeros((s1 - s0, 2))
    for l in layers:
        start, y = layer_sound(l, sounds, num, den, s1)
        for spec in l.get("fx", []):
            if not spec.get("on", True) or not in_range(spec):
                continue
            y = distort(y, start, spec, num, den)
        gain = 10 ** (l.get("gain", 0) / 20)
        for s in range(max(s0, start), s1):
            out[s - s0] += gain * y[s - start]
    return s0, out


def harmonics(m):
    """The 2nd and 3rd harmonics against the 1 kHz one, in dB (None below -100), left channel."""
    w = m[4800:24000, 0]
    t = np.arange(len(w))

    def size(h):
        return abs(np.sum(w * np.exp(-2j * np.pi * 1000 * h * t / WORK)))
    f = size(1)
    out = []
    for h in (2, 3):
        r = size(h) / f
        out.append(None if r <= 1e-5 else round(20 * math.log10(r), 6))
    return out


def db(v):
    return None if v == 0 else round(20 * math.log10(v), 6)


RAMP_PLACES = [RAMP_AT + (RAMP_LEN - 1) * j // 20 for j in range(21)]

CASES = []
for t in TYPES:
    CASES.append(("fx_dist_%03d" % (TYPES.index(t) + 1), "%s at the starting values (Drive 25%%, +10 dB into the curve): the sine's harmonics and the ramp's curve" % t.replace("_", " ").title(),
                  [{"file": "probe.wav", "fx": [{"type": t}]}], (24, 1), 0, 48, RAMP_PLACES, True))
CASES += [
    ("fx_dist_007", "Hard Clip, Drive 0: nothing above full level, so the sound is unchanged",
     [{"file": "probe.wav", "fx": [{"type": "hard_clip", "drive": 0}]}], (24, 1), 0, 48, [12000, 48000], False),
    ("fx_dist_008", "Gain 100 (four times the curve), Mix 50%, Volume 5.5 (half)",
     [{"file": "probe.wav", "fx": [{"gain": 100, "wet": 50, "volume": 5.5}]}], (24, 1), 0, 48, [12000, 48000], False),
    ("fx_dist_009", "Resolution 4 bits, Drive 0, Hard Clip: every sample on a step of 1/8",
     [{"file": "probe.wav", "fx": [{"type": "hard_clip", "drive": 0, "resolution": 4}]}], (24, 1), 0, 48, [12000, 48000], False),
    ("fx_dist_010", "Downsample 4, Drive 0, Hard Clip: each sample held for four",
     [{"file": "probe.wav", "fx": [{"type": "hard_clip", "drive": 0, "downsample": 4}]}], (24, 1), 0, 48, [12000, 48000], False),
    ("fx_dist_011", "Drive keyed from 0% (frame 0) to 100% (frame 10) on the sine: it grows steadily harsher",
     [{"file": "probe.wav", "fx": [{"drive": [[0, 0], [10, 100]]}]}], (24, 1), 0, 24, [0, 10000, 20000], False),
    ("fx_dist_012", "Downsample keyed from 1 (frame 0) to 32 (frame 12) at 29.97",
     [{"file": "probe.wav", "fx": [{"type": "hard_clip", "drive": 0, "downsample": [[0, 1], [12, 32]]}]}], (30000, 1001), 0, 24, [0, 9600, 19200], False),
    ("fx_dist_013", "frames 5 to 14 of fx_dist_010 alone: the same samples as the whole mix there (the hold counts from the layer's start)",
     [{"file": "probe.wav", "fx": [{"type": "hard_clip", "drive": 0, "downsample": 4}]}], (24, 1), 5, 10, [10000], False),
    ("fx_dist_014", "a layer starting at frame 7 with a level of -6 dB, Downsample 3: the hold counts from the layer's start, before the level",
     [{"file": "probe.wav", "in": 7, "gain": -6, "fx": [{"type": "hard_clip", "drive": 0, "downsample": 3}]}], (24, 1), 0, 48, [14000, 38000], False),
    ("fx_dist_015", "two in a row: Fuzz then Soft Clip",
     [{"file": "probe.wav", "fx": [{"type": "fuzz"}, {"type": "soft_clip"}]}], (24, 1), 0, 48, [12000, 48000], False),
    ("fx_dist_016", "switched off: the sound unchanged",
     [{"file": "probe.wav", "fx": [{"on": False}]}], (24, 1), 0, 48, [12000], False),
    ("fx_dist_017", "a Drive of 150, outside 0 to 100: kept, EFFECT_PARAMETER_INVALID, the sound unchanged",
     [{"file": "probe.wav", "fx": [{"drive": 150}]}], (24, 1), 0, 48, [12000], False),
    ("fx_dist_018", "a type of \"crunch\", which it does not have: kept, EFFECT_PARAMETER_INVALID, the sound unchanged",
     [{"file": "probe.wav", "fx": [{"type": "crunch"}]}], (24, 1), 0, 48, [12000], False),
]

REFUSED = [
    ("fx_dist_101", "Distortion on a solid layer is PROJECT_SCHEMA_INVALID",
     [{"solid": True, "effects_json": effects_json([{}])}]),
    ("fx_dist_102", "an expression on a Distortion number is PROJECT_SCHEMA_INVALID: sound effects take keys only",
     [{"file": "probe.wav", "effects_json": [dict(effects_json([{}])[0], parameters=dict(
         params_json({}), drive={"base": 25, "keyframes": [], "expression": "50"}))]}]),
    ("fx_dist_103", "a picture Mix on Distortion is PROJECT_SCHEMA_INVALID: its wet share is its own Mix setting",
     [{"file": "probe.wav", "effects_json": [dict(effects_json([{}])[0], mix=50)]}]),
]


def main():
    sounds = media()
    out = {"_source": "tools/distortion_reference.py (D-644)", "tolerance": 2e-5,
           "defaults": DEFAULTS, "ranges": RANGES, "types": TYPES, "cases": [], "refused": []}
    for case, what, layers, (num, den), a, k, extra, probe in CASES:
        duration = 48
        for l in layers:
            l.setdefault("out", duration)
            l["effects_json"] = effects_json(l["fx"])
        doc = project(case, layers, num, den, duration)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        s0, m = mix(copy.deepcopy(layers), sounds, num, den, a, k)
        at = places(s0, len(m), extra)
        peak = np.abs(m).max(axis=0)
        bad = not all(in_range(s) for l in layers for s in l["fx"])
        entry = {
            "project": case + ".json", "what": what, "from": a, "frames": k,
            "first_sample": s0, "samples": len(m),
            "peak_db": [db(float(peak[0])), db(float(peak[1]))],
            "warnings": ["EFFECT_PARAMETER_INVALID"] if bad else [], "places": at,
            "values": [[float(v) for v in m[i]] for i in at],
        }
        if probe:
            x, _ = sounds["probe.wav"]
            entry["harmonics_db"] = harmonics(m)
            entry["curve"] = [{"sample": s, "in": float(x[s, 0]), "out": float(m[s, 0])} for s in RAMP_PLACES]
        out["cases"].append(entry)
    for case, what, layers in REFUSED:
        for l in layers:
            l.setdefault("out", 48)
        doc = project(case, layers, 24, 1, 48)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        out["refused"].append({"project": case + ".json", "what": what, "diagnostic": "PROJECT_SCHEMA_INVALID"})
    (OUT / "expected_distortion.json").write_text(json.dumps(out, indent=1) + "\n", encoding="utf-8")
    print("wrote", len(CASES) + len(REFUSED), "cases")
    for c in out["cases"]:
        print(c["project"], c["peak_db"], c.get("harmonics_db"))


if __name__ == "__main__":
    main()
