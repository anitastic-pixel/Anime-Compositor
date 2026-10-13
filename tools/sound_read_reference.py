"""D-639 (S-1 of D-638): the fixtures for reading sound, written before the code.

Makes the test sounds with ffmpeg in `Fixtures/sound_read/media/`, decodes each back with
ffmpeg to 64-bit floats, and writes `Fixtures/sound_read/expected_sound_read.json`:

- for every file: its rate, channels and length, where its two clicks land (B-112's method: a
  click on the first sample of frames 2 and 12 at 24 frames a second, found as the loudest
  sample of each stretch), and ffmpeg's decode at named places;
- for files not at 48,000 a second, or not two channels: the working sound, 48,000 a second,
  two channels, worked out here by document 21's windowed-sinc rule (D-639), at named places,
  with its length and click positions;
- Opus: read by neither road (D-638 1 (a)), so refused with MEDIA_AUDIO_PREVIEW_ONLY.

The test sounds are written once and kept: an encoder of another version would make other
bytes, and the expected values are the decode of the bytes kept. Run with `--remake` to make
them again (a D-566 regeneration, in its own commit).

Run from the repository root: python tools/sound_read_reference.py [--remake]
Needs ffmpeg and numpy.
"""
import json
import math
import pathlib
import subprocess
import sys

import numpy as np

ROOT = pathlib.Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "sound_read"
MEDIA = OUT / "media"
WORK = 48000
FPS = 24
SECONDS = 1.5


def ffmpeg(*args):
    subprocess.run(["ffmpeg", "-v", "error", "-y", *args], check=True)


def decode(path):
    """ffmpeg's decode of the first sound track: (rate, channels, samples[n][ch])."""
    probe = subprocess.run(
        ["ffprobe", "-v", "error", "-select_streams", "a:0", "-show_entries",
         "stream=sample_rate,channels", "-of", "json", str(path)],
        check=True, capture_output=True, text=True).stdout
    s = json.loads(probe)["streams"][0]
    rate, ch = int(s["sample_rate"]), int(s["channels"])
    raw = subprocess.run(
        ["ffmpeg", "-v", "error", "-i", str(path), "-map", "0:a:0", "-f", "f64le", "-"],
        check=True, capture_output=True).stdout
    return rate, ch, np.frombuffer(raw, dtype="<f8").reshape(-1, ch)


def first_sample(n, rate):
    return n * rate // FPS


def source(rate, channels):
    """Silence with a 0.9 click on the first sample of frames 2 and 12, then from frame 15 a
    different sine in every channel (220 Hz times the channel's number plus one, 0.3)."""
    n = int(SECONDS * rate)
    x = np.zeros((n, channels))
    for f in (2, 12):
        x[first_sample(f, rate), :] = 0.9
    start = first_sample(15, rate)
    t = np.arange(n - start) / rate
    for c in range(channels):
        x[start:, c] = 0.3 * np.sin(2 * np.pi * 220 * (c + 1) * t)
    return x


def clicks(x, rate):
    """The loudest sample before frame 7 and between frames 7 and 15, the channels' mean."""
    m = np.abs(x.mean(axis=1))
    a, b = first_sample(7, rate), first_sample(15, rate)
    return [int(np.argmax(m[:a])), a + int(np.argmax(m[a:b]))]


def places(n, rate):
    """Every 1,500th sample, and two each side of both clicks, inside the file."""
    at = set(range(0, n, 1500))
    for f in (2, 12):
        c = first_sample(f, rate)
        at.update(range(c - 2, c + 3))
    return sorted(i for i in at if 0 <= i < n)


# Document 21, D-639: the working sound.
def stereo(x):
    """One channel plays in both; two stay; of more, 0 left, 1 right, 2 (centre) at 0.7071 to
    both, 3 (low frequencies) left out, 4 on to left (even) and right (odd) at 0.7071."""
    ch = x.shape[1]
    if ch == 1:
        return np.hstack([x, x])
    if ch == 2:
        return x.copy()
    h = math.sqrt(0.5)
    left, right = x[:, 0].copy(), x[:, 1].copy()
    if ch > 2:
        left += h * x[:, 2]
        right += h * x[:, 2]
    for c in range(4, ch):
        if c % 2 == 0:
            left += h * x[:, c]
        else:
            right += h * x[:, c]
    return np.stack([left, right], axis=1)


def working(x, rate):
    """48,000 a second: output j sits at t = j rate / 48000 input samples, and is
    sum over k of x[k] fc sinc(fc (t - k)) w((t - k) / W), fc = min(1, 48000 / rate), W = 32 / fc,
    w(u) = 0.42 + 0.5 cos(pi u) + 0.08 cos(2 pi u) for |u| < 1 and 0 outside; samples outside the
    file are 0. There are ceil(n 48000 / rate) outputs. At 48,000 nothing changes."""
    if rate == WORK:
        return x.copy()
    n = x.shape[0]
    out_n = -(-n * WORK // rate)
    fc = min(1.0, WORK / rate)
    w = 32.0 / fc
    y = np.zeros((out_n, x.shape[1]))
    for j in range(out_n):
        k0 = j * rate // WORK
        frac = (j * rate - k0 * WORK) / WORK
        lo, hi = k0 - int(w) - 1, k0 + int(w) + 2
        k = np.arange(max(lo, 0), min(hi, n))
        d = (k0 - k) + frac
        u = d / w
        g = np.where(np.abs(u) < 1, fc * np.sinc(fc * d) * (0.42 + 0.5 * np.cos(np.pi * u) + 0.08 * np.cos(2 * np.pi * u)), 0.0)
        y[j] = g @ x[k]
    return y


# name: (rate, channels, how ffmpeg writes it from the source WAV, lossless)
FILES = {
    "stereo_48k.wav": (48000, 2, ["-c:a", "pcm_s16le"], True),
    "stereo_48k.flac": (48000, 2, ["-c:a", "flac"], True),
    "stereo_48k.mp3": (48000, 2, ["-c:a", "libmp3lame", "-b:a", "192k"], False),
    "stereo_48k.ogg": (48000, 2, ["-c:a", "libvorbis", "-q:a", "5"], False),
    "stereo_48k.m4a": (48000, 2, ["-c:a", "aac", "-b:a", "192k"], False),
    "stereo_48k.aac": (48000, 2, ["-c:a", "aac", "-b:a", "192k"], False),
    "stereo_48k.mp4": (48000, 2, "video", False),
    "stereo_48k.mov": (48000, 2, "video", False),
    "mono_44k.wav": (44100, 1, ["-c:a", "pcm_s16le"], True),
    "mono_44k.flac": (44100, 1, ["-c:a", "flac"], True),
    "stereo_96k.flac": (96000, 2, ["-c:a", "flac"], True),
    "mono_8k.wav": (8000, 1, ["-c:a", "pcm_s16le"], True),
    "six_48k.flac": (48000, 6, ["-c:a", "flac"], True),
}
# Read by neither road: refused, the window still plays it.
REFUSED = {"stereo_48k.opus": ["-c:a", "libopus"]}


def make():
    MEDIA.mkdir(parents=True, exist_ok=True)
    made = {}
    for name, (rate, ch, how, _) in FILES.items():
        key = (rate, ch)
        if key not in made:
            wav = MEDIA / f"_src_{rate}_{ch}.wav"
            pcm = np.clip(np.round(source(rate, ch) * 32767), -32768, 32767).astype("<i2")
            write_wav(wav, pcm, rate)
            made[key] = wav
        src = made[key]
        target = MEDIA / name
        if name.endswith(".wav"):
            target.write_bytes(src.read_bytes())
        elif how == "video":
            ffmpeg("-f", "lavfi", "-i", f"color=c=gray:s=64x36:r={FPS}:d={SECONDS}", "-i", str(src),
                   "-map", "0:v", "-map", "1:a", "-c:v", "libx264", "-pix_fmt", "yuv420p",
                   "-c:a", "aac", "-b:a", "192k", "-shortest", str(target))
        else:
            ffmpeg("-i", str(src), *how, str(target))
    for name, how in REFUSED.items():
        ffmpeg("-i", str(made[(48000, 2)]), *how, str(MEDIA / name))
    for wav in made.values():
        wav.unlink()


def write_wav(path, pcm, rate):
    import wave
    with wave.open(str(path), "wb") as w:
        w.setnchannels(pcm.shape[1])
        w.setsampwidth(2)
        w.setframerate(rate)
        w.writeframes(pcm.tobytes())


def main():
    if "--remake" in sys.argv or not MEDIA.exists():
        make()
    version = subprocess.run(["ffmpeg", "-version"], capture_output=True, text=True).stdout.split("\n")[0]
    files = []
    for name, (rate, ch, _, lossless) in FILES.items():
        r, c, x = decode(MEDIA / name)
        assert (r, c) == (rate, ch), name
        at = places(len(x), r)
        row = {
            "file": name,
            "rate": r,
            "channels": c,
            "samples": len(x),
            "clicks": clicks(x, r),
            "lossless": lossless,
            "aac": name.endswith((".m4a", ".aac", ".mp4", ".mov")),
            # An MP4's edit list says where its sound ends; ffmpeg keeps the encoder's padding
            # after it. Either length is right: from the WAV's to ffmpeg's.
            "shortest": int(SECONDS * r) if name.endswith((".m4a", ".mp4", ".mov")) else len(x),
            "places": at,
            "values": [[float(x[i, k]) for k in range(c)] for i in at],
        }
        if (r, c) != (WORK, 2):
            y = working(stereo(x), r)
            wat = places(len(y), WORK)
            row["working"] = {
                "samples": len(y),
                "clicks": clicks(y, WORK),
                "places": wat,
                "values": [[float(y[i, 0]), float(y[i, 1])] for i in wat],
            }
        files.append(row)
    refused = [{"file": n, "diagnostic": "MEDIA_AUDIO_PREVIEW_ONLY"} for n in REFUSED]
    out = {
        "_source": "tools/sound_read_reference.py (D-639), decoded by " + version,
        "_rule": "values within 2e-5 for lossless files; for MP3, Vorbis and AAC within "
                 "lossy_bound, measured and proposed by B-518, clicks exact either way; length "
                 "from shortest to samples; places past the length read are not compared. A raw "
                 "AAC stream (.aac) carries no record of its encoder's start delay, so it is read "
                 "1,024 samples late, as ffmpeg reads it: a gap of the format, not of the reader",
        "lossy_bound": 0.02,
        "files": files,
        "refused": refused,
    }
    (OUT / "expected_sound_read.json").write_text(json.dumps(out, indent=1) + "\n", encoding="utf-8")
    for f in files:
        print(f["file"], f["rate"], f["channels"], f["samples"], f["clicks"],
              f.get("working", {}).get("samples"), f.get("working", {}).get("clicks"))


if __name__ == "__main__":
    main()
