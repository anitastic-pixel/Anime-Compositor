"""D-643 (S-5 of D-638): the fixtures for Compressor, written before the code.

Writes test sounds as 32-bit floating-point WAV files in `Fixtures/compressor/media/`, one
project per case, and `expected_compressor.json`. The rule is document 21's (D-643), ours;
After Effects 26.0 gives the controls and their starting values, not the maths:
- an audio layer's sound effects run, in order, on its sound from the layer's first frame,
  before its level (D-642);
- Compressor keeps a gain change g in dB, 0 at the layer's first sample. For each sample, with
  the louder channel's size p = max(|left|, |right|) and its level x = 20 log10(max(p, 1e-10)),
  the threshold T, the ratio R and the knee width W (dB, the whole width, centred on T), the
  wanted change c, with d = x - T, is
    2d < -W:                 c = 0;
    W > 0 and 2|d| <= W:     c = (1/R - 1) (d + W/2)^2 / (2W)   (the soft knee);
    else:                    c = (1/R - 1) d;
  then g moves toward c: when c < g it attacks, with a time of attack x 48 samples; otherwise it
  releases, with a time of release x 48 samples, or a tenth of that while Auto Release is on and
  g is more than 3 dB below c (our two-speed release: fast back from a deep cut, slow for the
  last 3 dB). With that time t, a = exp(-1/t) (0 when t is 0) and g = a g + (1 - a) c.
  Both channels are multiplied by 10^((g + makeup) / 20); then, if the louder channel is above
  the Output Limit L = 10^(limit / 20), both are scaled by L / that channel's size (an instant
  limiter that keeps the balance);
- a keyed number moves in a straight line across each frame's samples (as Noise Gate's, D-642);
- a Compressor switched off, or with a number outside its range or an Auto Release other than
  "on" or "off" (kept, with EFFECT_PARAMETER_INVALID), passes the sound unchanged.
The test sounds are 1 kHz square waves, whose size is the same on every sample, so the gain is
read exactly from the mix as out / in.
The curve table gives, for the staircase (-60 to 0 dB in 1 dB steps of 4,800 samples), the
input level and the steady gain change from the rule at each step's last sample.
The timing table gives, for each step of a step test, how many samples after the step the gain
(g + makeup, in dB, read off the mix) first covers 63% and 90% of the way from where it was
on the sample before the step to the curve's value for the new level.
Run from the repository root: python tools/compressor_reference.py
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
import noise_gate_reference as ng  # noqa: E402  D-642

OUT = ROOT / "Fixtures" / "compressor"
MEDIA = OUT / "media"
WORK = 48000
DEFAULTS = {"threshold": -16.0, "ratio": 3.0, "knee": 15.0, "attack": 6.0, "release": 440.0,
            "auto_release": "on", "makeup": 0.0, "output_limit": 0.0}
RANGES = {"threshold": (-60, 0), "ratio": (0.4, 30), "knee": (0, 30), "attack": (0, 400),
          "release": (1, 4000), "makeup": (-30, 30), "output_limit": (-30, 0)}
NUMBERS = ["threshold", "ratio", "knee", "attack", "release", "makeup", "output_limit"]
STEP = 4800


def square(n, db):
    k = np.arange(n)
    return 10 ** (db / 20) * np.where((k % 48) < 24, 1.0, -1.0)


def media():
    MEDIA.mkdir(parents=True, exist_ok=True)
    s = {}
    st = np.concatenate([square(STEP, -60 + i) for i in range(61)])
    s["staircase.wav"] = (write_wav(MEDIA / "staircase.wav", np.stack([st, st], 1), WORK), WORK)
    n = 4 * WORK
    lv = np.full(n, -40.0)
    lv[24000:72000] = 0.0
    k = np.arange(n)
    sq = np.where((k % 48) < 24, 1.0, -1.0)
    step = 10 ** (lv / 20) * sq
    s["step.wav"] = (write_wav(MEDIA / "step.wav", np.stack([step, step], 1), WORK), WORK)
    steady = square(2 * WORK, -20)
    s["steady.wav"] = (write_wav(MEDIA / "steady.wav", np.stack([steady, steady], 1), WORK), WORK)
    left = square(n, -40)
    s["right_step.wav"] = (write_wav(MEDIA / "right_step.wav", np.stack([left, step], 1), WORK), WORK)
    return s


def params_json(spec, defaults):
    p = {}
    for k, v in defaults.items():
        v = spec.get(k, v)
        if isinstance(v, list):
            p[k] = {"base": v[0][1], "keyframes": [{"frame": f, "value": x, "interp": "linear"} for f, x in v]}
        else:
            p[k] = v
    return p


def effects_json(fx):
    out = []
    for i, (kind, spec) in enumerate(fx):
        if kind == "comp":
            out.append({"instance_id": "comp-%d" % i, "type_id": "core.compressor", "enabled": spec.get("on", True),
                        "parameters": params_json(spec, DEFAULTS)})
        else:
            out.append({"instance_id": "gate-%d" % i, "type_id": "core.noise_gate", "enabled": spec.get("on", True),
                        "parameters": ng.gate_json(spec)})
    return out


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
    if spec.get("auto_release", "on") not in ("on", "off"):
        return False
    for k, (lo, hi) in RANGES.items():
        v = spec.get(k, DEFAULTS[k])
        for x in ([y for _, y in v] if isinstance(v, list) else [v]):
            if not lo <= x <= hi:
                return False
    return True


def curve(x, t, r, w):
    """The wanted gain change in dB at level x."""
    d = x - t
    if 2 * d < -w:
        return 0.0
    if w > 0 and 2 * abs(d) <= w:
        return (1 / r - 1) * (d + w / 2) ** 2 / (2 * w)
    return (1 / r - 1) * d


def compress(x, s0, spec, num, den):
    """Compressor over x, whose first row is composition sample s0. Gives (out, gains in dB)."""
    out = x.copy()
    gains = np.zeros(len(x))
    auto = spec.get("auto_release", "on") == "on"
    keyed = any(isinstance(spec.get(k), list) for k in NUMBERS)
    fixed = {k: float(spec.get(k, DEFAULTS[k])) for k in NUMBERS} if not keyed else None
    g = 0.0
    for i in range(len(x)):
        s = s0 + i
        v = fixed if fixed is not None else {k: number(spec, k, s, num, den) for k in NUMBERS}
        p = max(abs(x[i, 0]), abs(x[i, 1]))
        level = 20 * math.log10(max(p, 1e-10))
        c = curve(level, v["threshold"], v["ratio"], v["knee"])
        if c < g:
            t = v["attack"] * 48
        else:
            t = v["release"] * 48
            if auto and c - g > 3:
                t /= 10
        a = math.exp(-1 / t) if t > 0 else 0.0
        g = a * g + (1 - a) * c
        y = x[i] * 10 ** ((g + v["makeup"]) / 20)
        lim = 10 ** (v["output_limit"] / 20)
        q = max(abs(y[0]), abs(y[1]))
        if q > lim:
            y = y * (lim / q)
        out[i] = y
        gains[i] = g + v["makeup"]
    return out, gains


def mix(layers, sounds, num, den, a, k):
    """The composition's mix of frames a .. a + k, and the last compressor's gains (dB) and start."""
    s0, s1 = first_sample(a, num, den), first_sample(a + k, num, den)
    out = np.zeros((s1 - s0, 2))
    last = None
    for l in layers:
        start, y = layer_sound(l, sounds, num, den, s1)
        for kind, spec in l.get("fx", []):
            if not spec.get("on", True):
                continue
            if kind == "comp":
                if not in_range(spec):
                    continue
                y, gains = compress(y, start, spec, num, den)
                last = (start, gains)
            else:
                if not ng.in_range(spec):
                    continue
                y, _, _ = ng.gate(y, start, spec, num, den)
        gain = 10 ** (l.get("gain", 0) / 20)
        for s in range(max(s0, start), s1):
            out[s - s0] += gain * y[s - start]
    return s0, out, last


def steps_timing(gains, start, spec, sounds, file, steps):
    """For each step sample: samples until 63% and 90% of the way to the curve's value."""
    x, _ = sounds[file]
    rows = []
    for s in steps:
        i = s - start
        before = gains[i - 1]
        p = max(abs(x[s, 0]), abs(x[s, 1]))
        level = 20 * math.log10(max(p, 1e-10))
        target = curve(level, spec.get("threshold", DEFAULTS["threshold"]), spec.get("ratio", DEFAULTS["ratio"]),
                       spec.get("knee", DEFAULTS["knee"])) + spec.get("makeup", DEFAULTS["makeup"])
        row = {"step": s, "from_db": float(before), "to_db": float(target)}
        for share, key in [(0.63, "n63"), (0.90, "n90")]:
            n = None
            for j in range(i, len(gains)):
                if (gains[j] - before) / (target - before) >= share:
                    n = j - i
                    break
            row[key] = n
        rows.append(row)
    return rows


def db(v):
    return None if v == 0 else round(20 * math.log10(v), 6)


KEYED_THRESHOLD = [[0, -40], [10, -10]]
KEYED_RATIO = [[0, 1], [12, 20]]
HARD = {"knee": 0, "ratio": 4}

# (case, what it shows, layers, (num, den), first frame, frames, duration, samples to look around,
#  steps to time or None)
CASES = [
    ("fx_comp_001", "the starting values on the staircase: the steady gain curve, the soft knee from -23.5 to -8.5 dB",
     [{"file": "staircase.wav", "fx": [("comp", {})]}], (24, 1), 0, 150, 150, [], None),
    ("fx_comp_002", "Knee 0 and Ratio 4 on the staircase: a hard corner at -16 dB, a quarter of the rise above it",
     [{"file": "staircase.wav", "fx": [("comp", dict(HARD))]}], (24, 1), 0, 150, 150, [], None),
    ("fx_comp_003", "Ratio 0.4 (it expands above the threshold) and Release 1 ms: louder above -16 dB, held at full level by the Output Limit",
     [{"file": "staircase.wav", "fx": [("comp", {"ratio": 0.4, "release": 1})]}], (24, 1), 0, 150, 150, [], None),
    ("fx_comp_004", "a step from -40 to 0 dB and back, Knee 0, Ratio 4: the attack (6 ms) and the two-speed Auto Release",
     [{"file": "step.wav", "fx": [("comp", dict(HARD))]}], (24, 1), 0, 96, 100, [24000, 72000], [24000, 72000]),
    ("fx_comp_005", "the same step with Auto Release off: one slow release of 440 ms",
     [{"file": "step.wav", "fx": [("comp", dict(HARD, auto_release="off"))]}], (24, 1), 0, 96, 100, [24000, 72000],
     [24000, 72000]),
    ("fx_comp_006", "Attack 0: the cut is whole on the step's first sample",
     [{"file": "step.wav", "fx": [("comp", dict(HARD, attack=0))]}], (24, 1), 0, 96, 100, [24000], [24000, 72000]),
    ("fx_comp_007", "Makeup Gain +12 dB and Output Limit -6 dB: louder, the loud part held at -6 dB",
     [{"file": "step.wav", "fx": [("comp", dict(HARD, makeup=12, output_limit=-6))]}], (24, 1), 0, 96, 100,
     [24000, 72000], None),
    ("fx_comp_008", "Threshold keyed from -40 dB (frame 0) to -10 dB (frame 10) over a steady -20 dB: the cut eases off",
     [{"file": "steady.wav", "fx": [("comp", {"threshold": KEYED_THRESHOLD})]}], (24, 1), 0, 24, 48, [0, 4800, 12000], None),
    ("fx_comp_009", "Ratio keyed from 1 (frame 0) to 20 (frame 12) at 29.97 over a steady -20 dB",
     [{"file": "steady.wav", "fx": [("comp", {"ratio": KEYED_RATIO})]}], (30000, 1001), 0, 24, 48, [0, 9600, 19200], None),
    ("fx_comp_010", "frames 40 to 59 of fx_comp_004 alone: the same samples as the whole mix there",
     [{"file": "step.wav", "fx": [("comp", dict(HARD))]}], (24, 1), 40, 20, 100, [72000], None),
    ("fx_comp_011", "a layer starting at frame 6 with a level of -6 dB: the compressor counts from the layer's start, before the level",
     [{"file": "step.wav", "in": 6, "gain": -6, "fx": [("comp", dict(HARD))]}], (24, 1), 0, 96, 100, [36000], None),
    ("fx_comp_012", "the compressor switched off: the sound unchanged",
     [{"file": "step.wav", "fx": [("comp", dict(HARD, on=False))]}], (24, 1), 0, 48, 100, [24000], None),
    ("fx_comp_013", "a Noise Gate (Threshold -30 dB) then a compressor: the gate shuts the quiet part first",
     [{"file": "step.wav", "fx": [("gate", {"threshold": -30}), ("comp", dict(HARD))]}], (24, 1), 0, 96, 100,
     [24000, 72000], None),
    ("fx_comp_014", "a Ratio of 50, outside 0.4 to 30: kept, EFFECT_PARAMETER_INVALID, the sound unchanged",
     [{"file": "step.wav", "fx": [("comp", {"ratio": 50})]}], (24, 1), 0, 48, 100, [24000], None),
    ("fx_comp_015", "an Auto Release of \"sometimes\": kept, EFFECT_PARAMETER_INVALID, the sound unchanged",
     [{"file": "step.wav", "fx": [("comp", {"auto_release": "sometimes"})]}], (24, 1), 0, 48, 100, [24000], None),
    ("fx_comp_016", "the right channel's step drives both: the quiet left is turned down with it",
     [{"file": "right_step.wav", "fx": [("comp", dict(HARD))]}], (24, 1), 0, 96, 100, [24000, 72000], None),
]

REFUSED = [
    ("fx_comp_101", "Compressor on a solid layer is PROJECT_SCHEMA_INVALID",
     [{"solid": True, "effects_json": effects_json([("comp", {})])}]),
    ("fx_comp_102", "an expression on a Compressor number is PROJECT_SCHEMA_INVALID: sound effects take keys only",
     [{"file": "step.wav", "effects_json": [dict(effects_json([("comp", {})])[0], parameters=dict(
         params_json({}, DEFAULTS), ratio={"base": 3, "keyframes": [], "expression": "4"}))]}]),
    ("fx_comp_103", "a Mix on Compressor is PROJECT_SCHEMA_INVALID: a sound effect has no Mix",
     [{"file": "step.wav", "effects_json": [dict(effects_json([("comp", {})])[0], mix=50)]}]),
]


def main():
    sounds = media()
    out = {"_source": "tools/compressor_reference.py (D-643)", "tolerance": 2e-5,
           "defaults": DEFAULTS, "ranges": RANGES, "cases": [], "curves": {}, "refused": []}
    for case, what, layers, (num, den), a, k, duration, extra, steps in CASES:
        for l in layers:
            l.setdefault("out", duration)
            l["effects_json"] = effects_json(l["fx"])
        doc = project(case, layers, num, den, duration)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        s0, m, last = mix(copy.deepcopy(layers), sounds, num, den, a, k)
        at = places(s0, len(m), extra)
        peak = np.abs(m).max(axis=0)
        bad = not all(in_range(s) if kind == "comp" else ng.in_range(s) for l in layers for kind, s in l["fx"])
        entry = {
            "project": case + ".json", "what": what, "from": a, "frames": k,
            "first_sample": s0, "samples": len(m),
            "peak_db": [db(float(peak[0])), db(float(peak[1]))],
            "warnings": ["EFFECT_PARAMETER_INVALID"] if bad else [], "places": at,
            "values": [[float(v) for v in m[i]] for i in at],
            "timing": None,
        }
        spec = [s for kind, s in layers[0]["fx"] if kind == "comp"][0]
        if steps is not None:
            start, gains = last
            entry["timing"] = steps_timing(gains, start, spec, sounds, layers[0]["file"], steps)
        if layers[0]["file"] == "staircase.wav":
            x, _ = sounds["staircase.wav"]
            rows = []
            for i in range(61):
                s = (i + 1) * STEP - 1
                level = 20 * math.log10(abs(x[s, 0]))
                c = curve(level, spec.get("threshold", DEFAULTS["threshold"]), spec.get("ratio", DEFAULTS["ratio"]),
                          spec.get("knee", DEFAULTS["knee"]))
                rows.append({"sample": s, "in_db": level, "gain_db": c})
            entry["curve"] = rows
        out["cases"].append(entry)
    for case, what, layers in REFUSED:
        for l in layers:
            l.setdefault("out", 48)
        doc = project(case, layers, 24, 1, 48)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        out["refused"].append({"project": case + ".json", "what": what, "diagnostic": "PROJECT_SCHEMA_INVALID"})
    (OUT / "expected_compressor.json").write_text(json.dumps(out, indent=1) + "\n", encoding="utf-8")
    print("wrote", len(CASES) + len(REFUSED), "cases")
    for c in out["cases"]:
        print(c["project"], c["peak_db"], c["timing"])


if __name__ == "__main__":
    main()
