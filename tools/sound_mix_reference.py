"""D-640 (S-2 of D-638): the fixtures for mixing a composition's sound, written before the code.

Writes test sounds as 32-bit floating-point WAV files (by hand, with `struct`) in
`Fixtures/sound_mix/media/`, one project per case, and `expected_sound_mix.json`: for each case
the stretch of frames mixed, its length in samples at 48,000 a second, the mix at named places
in both channels, each channel's peak in decibels, and the warnings. The rule is document 21's
(D-640):

- composition sample s (48,000 a second) belongs to frame f, the last frame whose first sample
  floor(f x 48000 x den / num) is not after s (ADR-018);
- an audio layer that is on, and whose span holds f, is heard at f through its local frame
  n = f - in + offset: s reads sample first_sample(n) + (s - first_sample(f)) of the layer's
  working sound (document 21, D-639), silence before its first sample and after its last;
- times the layer's level, 10^(gain / 20); the layers are added, nothing is clipped;
- a stretch of frames a .. a + k is samples first_sample(a) .. first_sample(a + k).

A sound the core does not read (Opus) or cannot read is left out with its warning, and the
others are still heard. The Opus file is made by ffmpeg; only its name matters here.

Run from the repository root: python tools/sound_mix_reference.py
Needs numpy (and ffmpeg for the Opus file).
"""
import copy
import json
import math
import pathlib
import struct
import subprocess
import sys

import numpy as np

ROOT = pathlib.Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from sound_read_reference import stereo, working  # noqa: E402  document 21's working sound

OUT = ROOT / "Fixtures" / "sound_mix"
MEDIA = OUT / "media"
WORK = 48000
MINUS_6 = 10 ** (-6 / 20)


def write_wav(path, x, rate):
    """A floating-point WAV (format 3), samples x[n][ch] as 32-bit floats."""
    n, ch = x.shape
    data = x.astype("<f4").tobytes()
    fmt = struct.pack("<HHIIHH", 3, ch, rate, rate * ch * 4, ch * 4, 32)
    body = b"WAVE" + b"fmt " + struct.pack("<I", len(fmt)) + fmt + b"data" + struct.pack("<I", len(data)) + data
    path.write_bytes(b"RIFF" + struct.pack("<I", len(body)) + body)
    return x.astype("<f4").astype(np.float64)


def media():
    MEDIA.mkdir(parents=True, exist_ok=True)
    s = {}
    t = np.arange(WORK) / WORK
    sine = MINUS_6 * np.sin(2 * np.pi * 1000 * t)
    s["sine_1k.wav"] = (write_wav(MEDIA / "sine_1k.wav", np.stack([sine, sine], 1), WORK), WORK)
    n = int(1.5 * WORK)
    c = np.zeros((n, 2))
    for f in (2, 12):
        c[f * WORK // 24] = [0.9, -0.5]
    s["clicks.wav"] = (write_wav(MEDIA / "clicks.wav", c, WORK), WORK)
    i = np.arange(WORK) / WORK
    s["ramp.wav"] = (write_wav(MEDIA / "ramp.wav", np.stack([i, -i], 1), WORK), WORK)
    m = np.arange(22050) / 44100
    mono = (0.4 * np.sin(2 * np.pi * 300 * m))[:, None]
    s["mono_44k.wav"] = (write_wav(MEDIA / "mono_44k.wav", mono, 44100), 44100)
    if not (MEDIA / "sine.opus").exists():
        subprocess.run(["ffmpeg", "-v", "error", "-y", "-i", str(MEDIA / "sine_1k.wav"), "-c:a", "libopus",
                        str(MEDIA / "sine.opus")], check=True)
    return s


def first_sample(f, num, den):
    return (f * WORK * den) // num


def frame_of(s, num, den):
    f = (s * num) // (WORK * den)
    while first_sample(f + 1, num, den) <= s:
        f += 1
    while first_sample(f, num, den) > s:
        f -= 1
    return f


def project(case, layers, num, den, duration):
    assets, made = [], []
    for k, l in enumerate(layers):
        aid = "asset-%d" % k
        assets.append({"id": aid, "kind": "audio", "name": l["file"].split(".")[0], "path": "media/" + l["file"]})
        made.append({"id": "sound%d" % k, "kind": "audio", "name": "sound %d" % k, "asset_id": aid,
                     "enabled": l.get("on", True), "locked": False, "in_frame": l.get("in", 0),
                     "out_frame": l.get("out", duration), "source_offset_frames": l.get("offset", 0),
                     "gain_db": l.get("gain", 0)})
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


def mix(layers, sounds, num, den, a, k):
    s0, s1 = first_sample(a, num, den), first_sample(a + k, num, den)
    out = np.zeros((s1 - s0, 2))
    warnings = []
    for l in layers:
        if l["file"] not in sounds:
            warnings.append(l["warning"])
            continue
        if not l.get("on", True):
            continue
        x, rate = sounds[l["file"]]
        w = working(stereo(x), rate)
        g = 10 ** (l.get("gain", 0) / 20)
        for s in range(s0, s1):
            f = frame_of(s, num, den)
            if not (l.get("in", 0) <= f < l["out"]):
                continue
            n = f - l.get("in", 0) + l.get("offset", 0)
            i = first_sample(n, num, den) + (s - first_sample(f, num, den))
            if 0 <= i < len(w):
                out[s - s0] += g * w[i]
    return s0, out, warnings


def places(s0, n, extra):
    at = set(range(0, n, 997)) | {n - 1}
    for e in extra:
        at.update(range(e - s0 - 2, e - s0 + 3))
    return sorted(i for i in at if 0 <= i < n)


def db(v):
    return None if v == 0 else round(20 * math.log10(v), 6)


# (case, what it shows, layers, (num, den), first frame, frames, samples to look around)
CASES = [
    ("fx_smix_001", "a 1 kHz sine at -6 dB through a level of -6 dB peaks at -12.00 dB",
     [{"file": "sine_1k.wav", "gain": -6}], (24, 1), 0, 24, [12]),
    ("fx_smix_002", "the same layer switched off: every sample 0",
     [{"file": "sine_1k.wav", "gain": -6, "on": False}], (24, 1), 0, 24, []),
    ("fx_smix_003", "two layers add: the sine and the clicks, each at its level",
     [{"file": "sine_1k.wav"}, {"file": "clicks.wav", "gain": -3}], (24, 1), 0, 36, [4000, 24000]),
    ("fx_smix_004", "a layer starting at frame 5: silence to sample 10000, the file from there",
     [{"file": "ramp.wav", "in": 5}], (24, 1), 0, 30, [10000]),
    ("fx_smix_005", "a layer starting at frame 2 with its first 6 frames cut: sample 4000 is the file's 12000",
     [{"file": "ramp.wav", "in": 2, "offset": 6}], (24, 1), 0, 24, [4000, 40000]),
    ("fx_smix_006", "a layer ending at frame 10: silence from sample 20000",
     [{"file": "ramp.wav", "out": 10}], (24, 1), 0, 24, [20000]),
    ("fx_smix_007", "a layer whose sound starts 3 frames late (offset -3): silence to sample 6000",
     [{"file": "ramp.wav", "offset": -3}], (24, 1), 0, 24, [6000]),
    ("fx_smix_008", "29.97 frames a second, a layer starting at frame 1: each frame read from its own first sample",
     [{"file": "ramp.wav", "in": 1}], (30000, 1001), 0, 20, [1601, 3203, 4804, 6406]),
    ("fx_smix_009", "a level of +12 dB: the sine 3.98 times louder, past 1, not clipped",
     [{"file": "sine_1k.wav", "gain": 12}], (24, 1), 0, 12, [12]),
    ("fx_smix_010", "a level of -96 dB: the sine 63,000 times quieter",
     [{"file": "sine_1k.wav", "gain": -96}], (24, 1), 0, 12, [12]),
    ("fx_smix_011", "an Opus layer is left out with its warning; the sine beside it is heard",
     [{"file": "sine.opus", "warning": "MEDIA_AUDIO_PREVIEW_ONLY"}, {"file": "sine_1k.wav"}], (24, 1), 0, 12, [12]),
    ("fx_smix_012", "a layer whose file is gone is left out with its warning; the clicks beside it are heard",
     [{"file": "gone.wav", "warning": "MEDIA_AUDIO_UNREADABLE"}, {"file": "clicks.wav"}], (24, 1), 0, 24, [4000]),
    ("fx_smix_013", "a 44,100 mono file: its working sound, in both channels",
     [{"file": "mono_44k.wav"}], (24, 1), 0, 14, [24000]),
    ("fx_smix_014", "frames 10 to 14 of fx_smix_003 alone: the same samples as the whole mix there",
     [{"file": "sine_1k.wav"}, {"file": "clicks.wav", "gain": -3}], (24, 1), 10, 4, [24000]),
]


def main():
    sounds = media()
    out = {"_source": "tools/sound_mix_reference.py (D-640)", "tolerance": 2e-5, "cases": []}
    for case, what, layers, (num, den), a, k, extra in CASES:
        duration = 36
        for l in layers:
            l.setdefault("out", duration)
        doc = project(case, layers, num, den, duration)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        s0, m, warnings = mix(layers, sounds, num, den, a, k)
        at = places(s0, len(m), extra)
        peak = np.abs(m).max(axis=0) if len(m) else np.zeros(2)
        out["cases"].append({
            "project": case + ".json", "what": what, "from": a, "frames": k,
            "first_sample": s0, "samples": len(m), "silent": bool(not np.any(m)),
            "peak_db": [db(float(peak[0])), db(float(peak[1]))],
            "warnings": warnings, "places": at,
            "values": [[float(v) for v in m[i]] for i in at],
        })
    (OUT / "expected_sound_mix.json").write_text(json.dumps(out, indent=1) + "\n", encoding="utf-8")
    print("wrote", len(CASES), "cases")


if __name__ == "__main__":
    main()
