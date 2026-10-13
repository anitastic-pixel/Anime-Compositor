"""D-645 (S-7 of D-638): the fixtures for a keyed level, pan, and sound through composition
layers, written before the code.

Writes test sounds as 32-bit floating-point WAV files in `Fixtures/sound_layer/media/`, one
project per case, and `expected_sound_layer.json`. The rules are document 21's (D-645), ours:
- an audio layer's level `gain_db` is a number as before, or a keyed number
  {"base", "keyframes"}; each value from -96 to +12 dB, keys only (no expression);
- an audio layer's `pan` is absent (centre), a number, or a keyed number, each value from -100
  (all left) to +100 (all right), keys only. The balance law keeps the centre at unity:
  left x min(1, 1 - p/100), right x min(1, 1 + p/100);
- a keyed level or pan is read at frame f and f + 1 and moves in a straight line across frame
  f's samples (the level in dB), as a sound effect's keyed number does (D-642). Both act after
  the layer's sound effects;
- a composition layer that is switched on is heard: composition frame f, inside the layer's
  in and out points, reads inner frame g = floor(t), t the layer's source time (document 20:
  (f - in) x 100 / stretch + offset); the inner composition's sample first_inner(g) +
  (s - first(f)) is heard at sample s. Nothing is heard where g is outside the inner
  composition's frames, or where that sample is past the inner composition's last sample. At
  the same frame rate and a stretch of 100 this is the inner sound moved in time and nothing
  else. A composition layer has no level or pan of its own (a logged gap);
- a composition layer naming a composition that is not there is silent, with the opening
  warning COMPOSITION_REFERENCE_MISSING.
Clicks are single samples of 0.5, so a click's place says where the sound landed in time
(the click method).
Run from the repository root: python tools/sound_layer_reference.py
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
from noise_gate_reference import at_frame, gate, gate_json  # noqa: E402  D-642

OUT = ROOT / "Fixtures" / "sound_layer"
MEDIA = OUT / "media"
WORK = 48000
TRANSFORM = {"anchor": {"base": [0, 0], "keyframes": []},
             "position": {"base": [0, 0], "keyframes": []},
             "scale": {"base": [100, 100], "keyframes": []},
             "rotation": {"base": 0, "keyframes": []},
             "opacity": {"base": 1, "keyframes": []}}


def media():
    MEDIA.mkdir(parents=True, exist_ok=True)
    s = {}
    k = np.arange(2 * WORK)
    tone = 10 ** (-6 / 20) * np.sin(2 * np.pi * 1000 * k / WORK)
    s["tone.wav"] = (write_wav(MEDIA / "tone.wav", np.stack([tone, tone], 1), WORK), WORK)
    clicks = np.zeros(2 * WORK)
    clicks[::WORK // 2] = 0.5
    s["clicks.wav"] = (write_wav(MEDIA / "clicks.wav", np.stack([clicks, clicks], 1), WORK), WORK)
    return s


def keyed(v):
    """A number, or [[frame, value], ...] as a keyed number (linear keys)."""
    if isinstance(v, list):
        return {"base": v[0][1], "keyframes": [{"frame": f, "value": x, "interp": "linear"} for f, x in v]}
    return v


def layer_json(l, k, assets):
    if "comp" in l:
        m = {"id": "nest%d" % k, "kind": "composition", "name": "nest %d" % k, "enabled": l.get("on", True),
             "locked": False, "in_frame": l.get("in", 0), "out_frame": l["out"], "transform": TRANSFORM,
             "matte": None, "blend_mode": "normal", "effects": [], "masks": [],
             "composition_id": l["comp"], "source_offset_frames": l.get("offset", 0)}
        if "stretch" in l:
            m["time_stretch"] = l["stretch"]
        return m
    aid = "asset-" + l["file"].split(".")[0]
    if not any(a["id"] == aid for a in assets):
        assets.append({"id": aid, "kind": "audio", "name": l["file"].split(".")[0], "path": "media/" + l["file"]})
    m = {"id": "sound%d" % k, "kind": "audio", "name": "sound %d" % k, "asset_id": aid,
         "enabled": l.get("on", True), "locked": False, "in_frame": l.get("in", 0),
         "out_frame": l["out"], "source_offset_frames": l.get("offset", 0),
         "gain_db": l["gain_json"] if "gain_json" in l else keyed(l.get("gain", 0))}
    if "pan_json" in l:
        m["pan"] = l["pan_json"]
    elif "pan" in l:
        m["pan"] = keyed(l["pan"])
    if "gate" in l:
        m["effects"] = [{"instance_id": "gate-0", "type_id": "core.noise_gate", "enabled": True,
                         "parameters": gate_json(l["gate"])}]
    return m


def project(case, comps):
    assets, out = [], []
    for c in comps:
        made = [layer_json(l, k, assets) for k, l in enumerate(c["layers"])]
        out.append({
            "id": c["id"], "name": c["id"], "width": 4, "height": 2, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": c["rate"][0], "denominator": c["rate"][1]},
            "start_frame": c.get("start", 0), "duration_frames": c["duration"],
            "work_area": {"start_frame": c.get("start", 0), "end_frame_exclusive": c.get("start", 0) + c["duration"]},
            "layer_order": [m["id"] for m in made], "layers": made,
        })
    return {"schema_version": 0, "project_id": "proj-" + case,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": assets, "compositions": out}


def value(v, f):
    return at_frame(v, f) if isinstance(v, list) else float(v)


def audio_layer(l, sounds, num, den, s_end):
    """The layer as the composition hears it from its first sample to s_end, after its level."""
    x, rate = sounds[l["file"]]
    w = working(stereo(x), rate)
    lo, hi, off = l.get("in", 0), l["out"], l.get("offset", 0)
    start = first_sample(lo, num, den)
    y = np.zeros((max(0, s_end - start), 2))
    for s in range(start, s_end):
        f = frame_of(s, num, den)
        if not lo <= f < hi:
            continue
        i = first_sample(f - lo + off, num, den) + (s - first_sample(f, num, den))
        if 0 <= i < len(w):
            y[s - start] = w[i]
    if "gate" in l:
        y = gate(y, start, l["gate"], num, den)[0]
    gain, pan = l.get("gain", 0), l.get("pan", 0)
    for s in range(start, s_end):
        f = frame_of(s, num, den)
        s0, s1 = first_sample(f, num, den), first_sample(f + 1, num, den)
        t = (s - s0) / (s1 - s0)
        d = value(gain, f) + (value(gain, f + 1) - value(gain, f)) * t
        p = value(pan, f) + (value(pan, f + 1) - value(pan, f)) * t
        g = 10 ** (d / 20)
        y[s - start, 0] *= g * min(1.0, 1 - p / 100)
        y[s - start, 1] *= g * min(1.0, 1 + p / 100)
    return start, y


def mix(comps, sounds, cid, a, k):
    """Frames a to a + k of composition cid: (first sample, samples x 2)."""
    c = comps[cid]
    num, den = c["rate"]
    s0, s1 = first_sample(a, num, den), first_sample(a + k, num, den)
    out = np.zeros((s1 - s0, 2))
    for l in c["layers"]:
        if not l.get("on", True):
            continue
        if "comp" in l:
            inner = comps.get(l["comp"])
            if inner is None:
                continue
            ni, di = inner["rate"]
            start = inner.get("start", 0)
            end = start + inner["duration"]
            i0, i1 = first_sample(start, ni, di), first_sample(end, ni, di)
            _, m = mix(comps, sounds, l["comp"], start, end - start)
            lo, hi, off, stretch = l.get("in", 0), l["out"], l.get("offset", 0), l.get("stretch", 100)
            for s in range(s0, s1):
                f = frame_of(s, num, den)
                if not lo <= f < hi:
                    continue
                t = (f - lo + off) if stretch == 100 else (f - lo) * 100 / stretch + off
                g = math.floor(t)
                if not start <= g < end:
                    continue
                i = first_sample(g, ni, di) + (s - first_sample(f, num, den))
                if i0 <= i < i1:
                    out[s - s0] += m[i - i0]
            continue
        start, y = audio_layer(l, sounds, num, den, s1)
        for s in range(max(s0, start), s1):
            out[s - s0] += y[s - start]
    return s0, out


def db(v):
    return None if v == 0 else round(20 * math.log10(v), 6)


def main_comp(layers, rate=(24, 1), duration=48):
    return {"id": "comp-main", "rate": rate, "duration": duration, "layers": layers}


def inner_comp(cid, layers, rate=(24, 1), duration=48):
    return {"id": cid, "rate": rate, "duration": duration, "layers": layers}


TONE = {"file": "tone.wav", "out": 48}
CLICKS = {"file": "clicks.wav", "out": 48}
EDGES = [2000 * j for j in range(1, 48)]

CASES = [
    ("fx_slay_001", "a level keyed from -24 dB (frame 0) to 0 dB (frame 24): it rises steadily in decibels, sample by sample",
     [main_comp([dict(TONE, gain=[[0, -24], [24, 0]])])], 0, 48, EDGES[:13]),
    ("fx_slay_002", "a level keyed from 0 dB (frame 12) to -96 dB (frame 36), after a Noise Gate at its starting values",
     [main_comp([dict(TONE, gain=[[12, 0], [36, -96]], gate={})])], 0, 48, [24000, 48000, 72000]),
    ("fx_slay_003", "pan -100: all left, the right channel silent, the left at unity",
     [main_comp([dict(TONE, pan=-100)])], 0, 48, [12000]),
    ("fx_slay_004", "pan +50: the left at half, the right at unity",
     [main_comp([dict(TONE, pan=50)])], 0, 48, [12000]),
    ("fx_slay_005", "pan keyed from -100 (frame 0) to +100 (frame 24) at 29.97: it sweeps left to right",
     [main_comp([dict(TONE, pan=[[0, -100], [24, 100]])], rate=(30000, 1001))], 0, 48, [0, 19200, 38400]),
    ("fx_slay_006", "a level of -6 dB and pan -30, both plain numbers",
     [main_comp([dict(TONE, gain=-6, pan=-30)])], 0, 48, [12000]),
    ("fx_slay_007", "a composition layer showing a composition with the tone in it: the same sound as the tone on its own",
     [main_comp([{"comp": "comp-inner", "out": 48}]), inner_comp("comp-inner", [dict(TONE)])], 0, 48, [12000]),
    ("fx_slay_008", "a composition layer starting at frame 12: the inner composition's first click is heard at sample 24,000",
     [main_comp([{"comp": "comp-inner", "in": 12, "out": 48}]), inner_comp("comp-inner", [dict(CLICKS)])], 0, 48, [24000, 48000, 72000]),
    ("fx_slay_009", "a composition layer with a source offset of 6 frames: the inner click at 24,000 is heard at sample 12,000",
     [main_comp([{"comp": "comp-inner", "offset": 6, "out": 48}]), inner_comp("comp-inner", [dict(CLICKS)])], 0, 48, [0, 12000, 36000, 60000]),
    ("fx_slay_010", "the inner composition is 24 frames long and its sound layer runs on: nothing is heard after frame 24",
     [main_comp([{"comp": "comp-inner", "out": 48}]), inner_comp("comp-inner", [dict(TONE)], duration=24)], 0, 48, [48000]),
    ("fx_slay_011", "two deep: a composition at frame 6 inside one at frame 6, its clicks at -6 dB: the first click at sample 24,000, at 0.2506",
     [main_comp([{"comp": "comp-mid", "in": 6, "out": 48}]),
      inner_comp("comp-mid", [{"comp": "comp-inner", "in": 6, "out": 48}]),
      inner_comp("comp-inner", [dict(CLICKS, gain=-6)])], 0, 48, [24000, 48000, 72000]),
    ("fx_slay_012", "a composition layer switched off: silence",
     [main_comp([{"comp": "comp-inner", "on": False, "out": 48}]), inner_comp("comp-inner", [dict(TONE)])], 0, 48, []),
    ("fx_slay_013", "a 30 fps composition inside a 24 fps one: each outer frame plays its inner frame's samples onward (a logged gap)",
     [main_comp([{"comp": "comp-inner", "out": 48}]), inner_comp("comp-inner", [dict(CLICKS)], rate=(30, 1), duration=60)], 0, 48, [0, 24000, 48000]),
    ("fx_slay_014", "a composition layer at a stretch of 200: each inner frame is played for two outer frames (a logged gap)",
     [main_comp([{"comp": "comp-inner", "stretch": 200, "out": 48}]), inner_comp("comp-inner", [dict(TONE)])], 0, 48, [2000, 4000]),
    ("fx_slay_015", "frames 10 to 14 of fx_slay_001 alone: the same samples as the whole mix there",
     [main_comp([dict(TONE, gain=[[0, -24], [24, 0]])])], 10, 5, [22000]),
    ("fx_slay_016", "frames 20 to 29 of fx_slay_011 alone: the same samples as the whole mix there",
     [main_comp([{"comp": "comp-mid", "in": 6, "out": 48}]),
      inner_comp("comp-mid", [{"comp": "comp-inner", "in": 6, "out": 48}]),
      inner_comp("comp-inner", [dict(CLICKS, gain=-6)])], 20, 10, [48000]),
    ("fx_slay_017", "one composition shown twice, at frames 0 and 6: each click heard twice, 12,000 samples apart",
     [main_comp([{"comp": "comp-inner", "out": 48}, {"comp": "comp-inner", "in": 6, "out": 48}]),
      inner_comp("comp-inner", [dict(CLICKS)])], 0, 48, [0, 12000, 24000, 36000]),
    ("fx_slay_018", "a composition layer naming a composition that is not there: silent, COMPOSITION_REFERENCE_MISSING on opening",
     [main_comp([{"comp": "comp-gone", "out": 48}, dict(TONE, gain=-12)])], 0, 48, [12000]),
]

REFUSED = [
    ("fx_slay_101", "a keyed level with a key at +20 dB, above +12",
     [main_comp([dict(TONE, gain=[[0, 0], [24, 20]])])]),
    ("fx_slay_102", "pan 150, outside -100 to +100",
     [main_comp([dict(TONE, pan=150)])]),
    ("fx_slay_103", "an expression on the level: a level takes keys only",
     [main_comp([dict(TONE, gain_json={"base": 0, "keyframes": [], "expression": "-6"})])]),
    ("fx_slay_104", "pan keyed to -120",
     [main_comp([dict(TONE, pan=[[0, 0], [24, -120]])])]),
    ("fx_slay_105", "pan written as a word",
     [main_comp([dict(TONE, pan_json="left")])]),
]


def main():
    sounds = media()
    out = {"_source": "tools/sound_layer_reference.py (D-645)", "tolerance": 2e-5, "cases": [], "refused": []}
    for case, what, comps, a, k, extra in CASES:
        doc = project(case, comps)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        by_id = {c["id"]: c for c in copy.deepcopy(comps)}
        s0, m = mix(by_id, sounds, "comp-main", a, k)
        at = places(s0, len(m), extra)
        peak = np.abs(m).max(axis=0)
        nz = [int(i) for i in np.nonzero(np.abs(m[:, 0]) > 0.1)[0]] if "clicks" in json.dumps(doc) else None
        entry = {"project": case + ".json", "what": what, "from": a, "frames": k,
                 "first_sample": s0, "samples": len(m),
                 "peak_db": [db(float(peak[0])), db(float(peak[1]))],
                 "warnings": ["COMPOSITION_REFERENCE_MISSING"] if case == "fx_slay_018" else [],
                 "places": at, "values": [[float(v) for v in m[i]] for i in at]}
        if nz is not None:
            entry["clicks"] = [{"sample": s0 + i, "left": float(m[i, 0])} for i in nz]
        out["cases"].append(entry)
    for case, what, comps in REFUSED:
        doc = project(case, comps)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        out["refused"].append({"project": case + ".json", "what": what, "diagnostic": "PROJECT_SCHEMA_INVALID"})
    (OUT / "expected_sound_layer.json").write_text(json.dumps(out, indent=1) + "\n", encoding="utf-8")
    print("wrote", len(CASES) + len(REFUSED), "cases")
    for c in out["cases"]:
        print(c["project"], c["samples"], c["peak_db"], [(x["sample"], round(x["left"], 4)) for x in c.get("clicks", [])][:8])


if __name__ == "__main__":
    main()
