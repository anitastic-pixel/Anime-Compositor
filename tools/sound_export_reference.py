"""D-641 (S-3 of D-638): the fixtures for exporting a composition's sound, written before the code.

Writes test sounds as 32-bit floating-point WAV files in `Fixtures/sound_export/media/`, one
project per case, and `expected_sound_export.json`.

An MP4 carries the sound as AAC, which changes every value, so an MP4 is never compared value
for value: it is checked by the click method (ADR-018). Each case puts single-sample clicks
(0.9 left, -0.5 right) on chosen frames by starting a layer of `click.wav`, whose first sample is
the click, on that frame. Decoded by ffmpeg 8.1, the loudest sample near each click must be
exactly sample first_sample(f) - first_sample(a) of the file, where a is the first frame
exported and first_sample(f) = floor(f x 48000 x den / num).

A WAV beside a sequence holds the mix of document 21 (D-640) as 24-bit integers: each value kept
between -1 and 1, times 8,388,607, rounded half away from nought. It is compared value for value
at named places, and the count of values past full level is given.

Run from the repository root: python tools/sound_export_reference.py
Needs numpy (and ffmpeg for the Opus file).
"""
import json
import math
import pathlib
import subprocess
import sys

import numpy as np

ROOT = pathlib.Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from sound_mix_reference import first_sample, mix, places, write_wav  # noqa: E402  D-640's mix

OUT = ROOT / "Fixtures" / "sound_export"
MEDIA = OUT / "media"
WORK = 48000
FULL = 8388607


def media():
    MEDIA.mkdir(parents=True, exist_ok=True)
    s = {}
    c = np.zeros((WORK // 2, 2))
    c[0] = [0.9, -0.5]
    s["click.wav"] = (write_wav(MEDIA / "click.wav", c, WORK), WORK)
    t = np.arange(WORK) / WORK
    sine = 10 ** (-6 / 20) * np.sin(2 * np.pi * 1000 * t)
    s["sine_1k.wav"] = (write_wav(MEDIA / "sine_1k.wav", np.stack([sine, sine], 1), WORK), WORK)
    i = np.arange(WORK) / WORK
    s["ramp.wav"] = (write_wav(MEDIA / "ramp.wav", np.stack([i, -i], 1), WORK), WORK)
    # The core does not read Opus (D-639), so the layer is left out; made by ffmpeg.
    if not (MEDIA / "voice.opus").exists():
        subprocess.run(["ffmpeg", "-v", "error", "-y", "-i", str(MEDIA / "sine_1k.wav"), "-c:a", "libopus",
                        str(MEDIA / "voice.opus")], check=True)
    return s


def project(case, layers, num, den, duration):
    assets, made = [], []
    for k, l in enumerate(layers):
        aid = "asset-%d" % k
        assets.append({"id": aid, "kind": "audio", "name": l["file"].split(".")[0], "path": "media/" + l["file"]})
        made.append({"id": "sound%d" % k, "kind": "audio", "name": "sound %d" % k, "asset_id": aid,
                     "enabled": True, "locked": False, "in_frame": l.get("in", 0),
                     "out_frame": duration, "source_offset_frames": 0, "gain_db": l.get("gain", 0)})
    return {
        "schema_version": 0,
        "project_id": "proj-" + case,
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": assets,
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": 64, "height": 64, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": num, "denominator": den}, "start_frame": 0,
            "duration_frames": duration,
            "work_area": {"start_frame": 0, "end_frame_exclusive": duration},
            "layer_order": [m["id"] for m in made], "layers": made,
        }],
    }


def clicks(frames):
    return [{"file": "click.wav", "in": f} for f in frames]


# MP4 cases: (case, what, layers, (num, den), first frame, last frame, click frames, warnings)
FILMS = [
    ("fx_sexp_001", "24 a second: clicks on frames 2, 12 and 30", clicks([2, 12, 30]), (24, 1), 0, 35,
     [2, 12, 30], []),
    ("fx_sexp_002", "29.97 a second: clicks on frames 3 and 25, samples 4804 and 40040", clicks([3, 25]),
     (30000, 1001), 0, 39, [3, 25], []),
    ("fx_sexp_003", "25 a second: clicks on frames 1 and 24, samples 1920 and 46080", clicks([1, 24]),
     (25, 1), 0, 29, [1, 24], []),
    ("fx_sexp_004", "frames 6 to 29 exported: the clicks on frames 8 and 20 are samples 4000 and 28000 of the file",
     clicks([8, 20]), (24, 1), 6, 29, [8, 20], []),
    ("fx_sexp_005", "an Opus layer is left out with its warning; the click beside it lands",
     [{"file": "voice.opus"}] + clicks([5]), (24, 1), 0, 23, [5], ["MEDIA_AUDIO_PREVIEW_ONLY"]),
]

# A composition with no sound layer: its MP4 has the one picture track it always had.
SILENT = ("fx_sexp_006", "no sound layer: the MP4 has its picture track and no sound track", (24, 1), 0, 11)

# WAV cases: (case, what, layers, (num, den), format, naming, first, last, look around)
WAVS = [
    ("fx_sexp_007", "a PNG sequence with its WAV: the sine at -6 dB and a click at -6 dB on frame 2",
     [{"file": "sine_1k.wav"}, {"file": "click.wav", "in": 2, "gain": -6}], (24, 1), "png", "shot_%04d.png", 0, 23, [4000]),
    ("fx_sexp_008", "an animated PNG at 29.97 with its WAV: the ramp from frame 1, frames 0 to 19",
     [{"file": "ramp.wav", "in": 1}], (30000, 1001), "apng", "film.png", 0, 19, [1601, 3203]),
    ("fx_sexp_009", "an EXR sequence of frames 4 to 15 with its WAV: the ramp, from sample 8000",
     [{"file": "ramp.wav"}], (24, 1), "exr-half", "plate.%04d.exr", 4, 15, [8000, 12000]),
    ("fx_sexp_010", "a level of +12 dB goes past full level: those values are written at full level, with a warning",
     [{"file": "sine_1k.wav", "gain": 12}], (24, 1), "png", "loud_%04d.png", 0, 11, [12]),
]

# A GIF with the WAV asked for: no WAV, since a GIF carries no sound.
GIF = ("fx_sexp_011", "a GIF with the WAV asked for writes no WAV", [{"file": "sine_1k.wav"}], (24, 1), 0, 5)


def wav_name(naming):
    stem = naming[:naming.index("%")] if "%" in naming else naming.rsplit(".", 1)[0]
    stem = stem.rstrip("_-. ")
    return (stem or "sound") + ".wav"


def int24(v):
    v = min(max(v, -1.0), 1.0) * FULL
    return int(math.floor(abs(v) + 0.5)) * (1 if v >= 0 else -1)


def main():
    sounds = media()
    out = {"_source": "tools/sound_export_reference.py (D-641)", "tolerance": 2e-5,
           "films": [], "wavs": []}
    for case, what, layers, (num, den), a, b, at, warnings in FILMS:
        doc = project(case, layers, num, den, b + 1)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        s0 = first_sample(a, num, den)
        out["films"].append({
            "project": case + ".json", "what": what, "first_frame": a, "last_frame": b,
            "samples": first_sample(b + 1, num, den) - s0,
            "clicks": [{"frame": f, "sample": first_sample(f, num, den) - s0} for f in at],
            "warnings": warnings, "codec": "aac", "sample_rate": WORK, "channels": 2,
            "bits_a_second": 192000,
        })
    case, what, (num, den), a, b = SILENT
    (OUT / (case + ".json")).write_text(json.dumps(project(case, [], num, den, b + 1), indent=2) + "\n",
                                        encoding="utf-8")
    out["silent"] = {"project": case + ".json", "what": what, "first_frame": a, "last_frame": b,
                     "sound_tracks": 0, "picture_tracks": 1}
    for case, what, layers, (num, den), fmt, naming, a, b, extra in WAVS:
        doc = project(case, layers, num, den, b + 1)
        (OUT / (case + ".json")).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
        for l in layers:
            l.setdefault("out", b + 1)
        s0, m, _ = mix(layers, sounds, num, den, a, b - a + 1)
        at = places(s0, len(m), extra)
        out["wavs"].append({
            "project": case + ".json", "what": what, "format": fmt, "naming": naming,
            "first_frame": a, "last_frame": b, "file": wav_name(naming),
            "rate": WORK, "channels": 2, "bits": 24, "samples": len(m),
            "past_full": int(np.sum(np.abs(m) > 1.0)),
            "places": at, "values": [[int24(float(v)) for v in m[i]] for i in at],
        })
    case, what, layers, (num, den), a, b = GIF
    (OUT / (case + ".json")).write_text(json.dumps(project(case, layers, num, den, b + 1), indent=2) + "\n",
                                        encoding="utf-8")
    out["gif"] = {"project": case + ".json", "what": what, "first_frame": a, "last_frame": b,
                  "naming": "preview.gif", "wav_written": False}
    (OUT / "expected_sound_export.json").write_text(json.dumps(out, indent=1) + "\n", encoding="utf-8")
    print("wrote", len(FILMS) + len(WAVS) + 2, "cases")


if __name__ == "__main__":
    main()
