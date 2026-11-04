"""D-1089: the fixtures for Convert Audio to Keyframes, written before the code.

After Effects' Animation > Keyframe Assistant > Convert Audio to Keyframes (tutorial
k6xZUlhlZxs at 1:23) adds a null "Audio Amplitude" with three Slider Controls, "Left Channel",
"Right Channel" and "Both Channels", keyed on every frame. The rule is document 21's (D-1089):

- the composition's sound is mixed as D-640 and D-645 mix it (every audio layer that is on, at
  its level), over the work area, or the whole composition when it has none;
- frame f's samples are first_sample(f) .. first_sample(f + 1) at 48,000 a second (ADR-018);
- Left Channel at f is 100 times the mean of the left channel's absolute values there, Right
  Channel the same of the right, Both Channels the mean of the two;
- one linear key per frame on each slider.

The mix is tools/sound_mix_reference.py's own, which is checked against FX-SOUND-MIX; this file
adds only the per-frame amplitude. Writes Fixtures/audio_keyframes/: the test sounds in media/
(32-bit floating-point WAV by hand), one project per case, and expected_audio_keyframes.json.

Run from the repository root: python tools/audio_keyframes_reference.py
Needs numpy.
"""
import json
import pathlib
import sys

import numpy as np

ROOT = pathlib.Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from sound_mix_reference import first_sample, mix, project, write_wav  # noqa: E402  D-640's mix

OUT = ROOT / "Fixtures" / "audio_keyframes"
MEDIA = OUT / "media"
WORK = 48000


def media():
    """beat.wav: 2 s at 48,000, the left a 100 Hz sine at 0.8 for frames 0-5, 12-17, 24-29 and
    36-41 at 24 a second and silent between, the right a steady 50 Hz sine at 0.3. hum_44k.wav:
    1 s mono at 44,100, a 200 Hz sine at 0.5, so its working sound is resampled (D-639)."""
    MEDIA.mkdir(parents=True, exist_ok=True)
    s = {}
    t = np.arange(2 * WORK) / WORK
    beat = (np.floor(t * 24) % 12 < 6) * 0.8 * np.sin(2 * np.pi * 100 * t)
    hum = 0.3 * np.sin(2 * np.pi * 50 * t)
    s["beat.wav"] = (write_wav(MEDIA / "beat.wav", np.stack([beat, hum], 1), WORK), WORK)
    m = np.arange(44100) / 44100
    s["hum_44k.wav"] = (write_wav(MEDIA / "hum_44k.wav", (0.5 * np.sin(2 * np.pi * 200 * m))[:, None], 44100), 44100)
    return s


# (case, what it shows, layers, (num, den), work area or None, duration)
CASES = [
    ("fx_akf_001", "the beat: Left Channel jumps on the beats, Right Channel steady, Both their mean",
     [{"file": "beat.wav"}], (24, 1), None, 48),
    ("fx_akf_002", "the beat at -6 dB starting at frame 6: nothing before frame 6, then half as loud",
     [{"file": "beat.wav", "in": 6, "gain": -6}], (24, 1), None, 48),
    ("fx_akf_003", "the beat and a 44,100 mono hum together: the hum adds to both channels",
     [{"file": "beat.wav"}, {"file": "hum_44k.wav"}], (24, 1), None, 24),
    ("fx_akf_004", "a work area of frames 12 to 36: keys on those 24 frames only",
     [{"file": "beat.wav"}], (24, 1), (12, 36), 48),
    ("fx_akf_005", "29.97 frames a second: each frame's own samples, 1601 or 1602 of them",
     [{"file": "beat.wav"}], (30000, 1001), None, 30),
    ("fx_akf_006", "the only audio layer switched off: nothing to hear, so the command is refused",
     [{"file": "beat.wav", "on": False}], (24, 1), None, 24),
]


def keys(m, s0, num, den, a, k):
    out = []
    for f in range(a, a + k):
        x = m[first_sample(f, num, den) - s0:first_sample(f + 1, num, den) - s0]
        left, right = 100 * float(np.mean(np.abs(x[:, 0]))), 100 * float(np.mean(np.abs(x[:, 1])))
        out.append([left, right, (left + right) / 2])
    return out


def main():
    sounds = media()
    out = {"_source": "tools/audio_keyframes_reference.py (D-1089)", "tolerance": 0.002, "cases": []}
    for case, what, layers, (num, den), area, duration in CASES:
        for l in layers:
            l.setdefault("out", duration)
        doc = project(case, layers, num, den, duration)
        a, b = area or (0, duration)
        doc["compositions"][0]["work_area"] = {"start_frame": a, "end_frame_exclusive": b}
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        heard = any(l.get("on", True) for l in layers)
        row = {"project": case + ".json", "what": what, "from": a, "frames": b - a, "refused": not heard}
        if heard:
            s0, m, _ = mix(layers, sounds, num, den, a, b - a)
            row["keys"] = keys(m, s0, num, den, a, b - a)
        out["cases"].append(row)
        print(case, "refused" if not heard else "frames %d..%d, frame %d: %s" % (
            a, b - 1, a, ", ".join("%.4f" % v for v in row["keys"][0])))
    (OUT / "expected_audio_keyframes.json").write_text(json.dumps(out, indent=1) + "\n", encoding="utf-8")
    print("wrote", len(CASES), "cases")


if __name__ == "__main__":
    main()
