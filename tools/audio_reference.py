"""Reference audio, worked a second way (D-71, ADR-018).

An audio layer draws nothing. What there is to pin is arithmetic and file reading:

**Time.** A composition's frame rate is a fraction, `numerator / denominator`, and a sound file
has a whole number of samples a second. Frame `n` of a file played from its start begins at

    first_sample(n) = floor(n * rate * denominator / numerator)

worked in whole numbers, never in seconds held as a float. Frame `n` is the samples from
`first_sample(n)` up to, not including, `first_sample(n + 1)`. On a layer, `n` is document 20's
local frame, `composition_frame - in_frame + source_offset_frames`, the same sum a cel uses.
Before the layer's in frame, from its out frame, before sample 0 and past the file's last sample
there is silence.

**Length.** A file of `samples` samples covers `ceil(samples * numerator / (rate * denominator))`
frames: every frame that holds at least one sample.

**WAV.** The header is read by hand here with `struct`, chunk by chunk, and never through
Python's `wave` module, so that the build's reader and this one share no code and no library.

**This file never runs the build's code path.**

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/audio_reference.py
"""

import json
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import adjust_reference as adj  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "audio"


def first_sample(n, rate, num, den):
    return (n * rate * den) // num


def length_frames(samples, rate, num, den):
    return -((-samples * num) // (rate * den))


# ---- time ------------------------------------------------------------------------------------

TIME = {
    "FX-AUD-001": ("24 frames a second and 48000 samples a second: 2000 samples to every frame.",
                   48000, 24, 1),
    "FX-AUD-002": ("24 frames a second and 44100 samples: 1837.5 to a frame, so frames take 1837 "
                   "and 1838 in turn and no sample is played twice or dropped.", 44100, 24, 1),
    "FX-AUD-003": ("24000/1001 frames a second and 48000 samples: 2002 to every frame, exactly.",
                   48000, 24000, 1001),
    "FX-AUD-004": ("24000/1001 frames a second and 44100 samples: 1839.3375 to a frame.",
                   44100, 24000, 1001),
}
TIME_FRAMES = [0, 1, 2, 3, 4, 23, 24, 1000, 86400]

# A layer: in, out, source offset, file length in samples; and the composition frames asked.
LAYER = {
    "FX-AUD-005": ("A layer that starts at frame 10 plays the file's first sample on frame 10.",
                   dict(in_frame=10, out_frame=40, source_offset_frames=0, samples=48000)),
    "FX-AUD-006": ("A source offset of 5 starts the layer five frames into the file.",
                   dict(in_frame=10, out_frame=40, source_offset_frames=5, samples=48000)),
    "FX-AUD-007": ("A negative source offset: the layer is silent until the file's first sample "
                   "comes round, three frames after the layer's in frame.",
                   dict(in_frame=10, out_frame=40, source_offset_frames=-3, samples=48000)),
    "FX-AUD-008": ("A file shorter than its layer: silence from the frame after its last sample. "
                   "47000 samples end part of the way through the 24th frame, which plays what "
                   "there is.",
                   dict(in_frame=0, out_frame=40, source_offset_frames=0, samples=47000)),
}
LAYER_FRAMES = [0, 9, 10, 11, 12, 13, 14, 23, 24, 33, 34, 39, 40]


def heard(layer, frame, rate=48000, num=24, den=1):
    """[first sample, one past the last] of the file heard on a composition frame, or None."""
    if not layer["in_frame"] <= frame < layer["out_frame"]:
        return None
    n = frame - layer["in_frame"] + layer["source_offset_frames"]
    if n < 0:
        return None
    a = first_sample(n, rate, num, den)
    b = min(first_sample(n + 1, rate, num, den), layer["samples"])
    return [a, b] if a < b else None


# ---- WAV -------------------------------------------------------------------------------------

def chunk(name, body):
    return name + struct.pack("<I", len(body)) + body + (b"\0" if len(body) % 2 else b"")


def fmt(tag, channels, rate, bits, extensible_tag=None):
    align = channels * bits // 8
    body = struct.pack("<HHIIHH", 0xFFFE if extensible_tag else tag, channels, rate, rate * align,
                       align, bits)
    if extensible_tag:
        guid = struct.pack("<H", extensible_tag) + bytes.fromhex("000000001000800000aa00389b71")
        body += struct.pack("<HHI", 22, bits, 0) + guid
    return chunk(b"fmt ", body)


def riff(*chunks, form=b"RIFF"):
    body = b"WAVE" + b"".join(chunks)
    return form + struct.pack("<I", len(body)) + body


def tone(samples, channels, bits, floating=False):
    """A ramp, not a sine: every byte is a whole number worked without a float library."""
    out = bytearray()
    for i in range(samples):
        for c in range(channels):
            v = ((i * 37 + c * 11) % 200) - 100  # -100..99
            if floating:
                out += struct.pack("<f" if bits == 32 else "<d", v / 128)
            elif bits == 8:
                out += struct.pack("<B", v + 128)
            elif bits == 16:
                out += struct.pack("<h", v * 256)
            elif bits == 24:
                out += struct.pack("<i", v * 65536)[:3]
            else:
                out += struct.pack("<i", v * 16777216)
    return bytes(out)


def wav_files():
    data = lambda s, ch, bits, fl=False: chunk(b"data", tone(s, ch, bits, fl))  # noqa: E731
    short = tone(480, 1, 16)
    files = {
        "pcm16_mono_48k.wav": riff(fmt(1, 1, 48000, 16), data(4800, 1, 16)),
        "pcm24_stereo_44k.wav": riff(fmt(1, 2, 44100, 24), data(4410, 2, 24)),
        "pcm8_mono_8k.wav": riff(fmt(1, 1, 8000, 8), data(801, 1, 8)),
        "pcm32_mono_48k.wav": riff(fmt(1, 1, 48000, 32), data(480, 1, 32)),
        "float32_stereo_48k.wav": riff(fmt(3, 2, 48000, 32), data(480, 2, 32, True)),
        "float64_mono_96k.wav": riff(fmt(3, 1, 96000, 64), data(960, 1, 64, True)),
        "extensible_6ch_48k.wav": riff(fmt(1, 6, 48000, 16, extensible_tag=1), data(480, 6, 16)),
        # A LIST chunk of odd length before `fmt `, so its pad byte must be stepped over, and an
        # unknown chunk after the sound.
        "chunks_in_the_way.wav": riff(chunk(b"LIST", b"INFOabc"), fmt(1, 1, 48000, 16),
                                      chunk(b"data", short), chunk(b"xtra", b"12345")),
        # `data` says 4800 samples and the file stops after 480 of them: a recording cut short.
        "cut_short.wav": riff(fmt(1, 1, 48000, 16),
                              b"data" + struct.pack("<I", 9600) + short),
        "no_sound.wav": riff(fmt(1, 1, 48000, 16), chunk(b"data", b"")),
        "adpcm.wav": riff(chunk(b"fmt ", struct.pack("<HHIIHH", 2, 1, 22050, 11155, 512, 4)
                                + struct.pack("<H", 0)), chunk(b"data", bytes(512))),
        "rf64.wav": riff(fmt(1, 1, 48000, 16), chunk(b"data", short), form=b"RF64"),
        "no_fmt.wav": riff(chunk(b"data", short)),
        "no_data.wav": riff(fmt(1, 1, 48000, 16)),
        "not_a_wav.wav": b"ID3\x03" + bytes(60),
        "zero_rate.wav": riff(fmt(1, 1, 0, 16), chunk(b"data", short)),
    }
    return files


def read_wav(raw):
    """What D-71 says a WAV header means. Returns a dict, or {"refused": why}."""
    if len(raw) < 12 or raw[:4] != b"RIFF" or raw[8:12] != b"WAVE":
        return {"refused": "not a RIFF WAVE file"}
    at, form, sound = 12, None, None
    while at + 8 <= len(raw):
        name, size = raw[at:at + 4], struct.unpack("<I", raw[at + 4:at + 8])[0]
        body = raw[at + 8:at + 8 + size]
        if name == b"fmt " and form is None:
            form = body
        elif name == b"data" and sound is None:
            sound = (size, len(body))
        at += 8 + size + (size % 2)
    if form is None or len(form) < 16:
        return {"refused": "no format chunk"}
    if sound is None:
        return {"refused": "no data chunk"}
    tag, channels, rate, _, align, bits = struct.unpack("<HHIIHH", form[:16])
    if tag == 0xFFFE and len(form) >= 26:
        tag = struct.unpack("<H", form[24:26])[0]
    if rate == 0 or channels == 0:
        return {"refused": "no sample rate or no channels"}
    if tag not in (1, 3):
        return {"encoding": "other", "channels": channels, "sample_rate": rate}
    if align != channels * bits // 8 or (tag, bits) not in {(1, 8), (1, 16), (1, 24), (1, 32),
                                                             (3, 32), (3, 64)}:
        return {"refused": "a sample size this build does not read"}
    said, there = sound
    out = {"encoding": "pcm" if tag == 1 else "float", "channels": channels,
           "sample_rate": rate, "bits": bits, "samples": there // align}
    if there < said:
        out["warning"] = "MEDIA_AUDIO_CUT_SHORT"
    return out


# ---- a project -------------------------------------------------------------------------------

def audio_layer(**more):
    return {"id": "sound", "kind": "audio", "name": "sound", "asset_id": "asset-sound",
            "enabled": True, "locked": False, "in_frame": 0, "out_frame": 3,
            "source_offset_frames": 0, "gain_db": 0, **more}


def project(fx, with_sound):
    p = adj.project_json(fx, {"layers": [adj.BG, adj.HALF]})
    if with_sound:
        p["assets"].append({"id": "asset-sound", "kind": "audio", "name": "sound",
                            "path": "media/pcm16_mono_48k.wav"})
        comp = p["compositions"][0]
        comp["layers"].insert(1, audio_layer())
        comp["layer_order"].insert(1, "sound")
    return p


# What a file may not say. Each is the project of FX-AUD-020 with one thing changed.
REFUSED = {
    "FX-AUD-030": ("An audio layer with a transform: it has no place to be.",
                   lambda l: l.update(transform={})),
    "FX-AUD-031": ("An audio layer with effects.", lambda l: l.update(effects=[])),
    "FX-AUD-032": ("An audio layer with a blend mode.", lambda l: l.update(blend_mode="normal")),
    "FX-AUD-033": ("An audio layer whose asset is a picture.",
                   lambda l: l.update(asset_id="asset-bg")),
    "FX-AUD-034": ("A level above +12 dB.", lambda l: l.update(gain_db=12.5)),
    "FX-AUD-035": ("A level that is not a number.", lambda l: l.update(gain_db="loud")),
}


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    expected = {"cases": {}}
    c = expected["cases"]

    for fx, (says, rate, num, den) in TIME.items():
        rows = {str(n): first_sample(n, rate, num, den) for n in TIME_FRAMES}
        c[fx] = {"says": says, "sample_rate": rate, "frame_rate": [num, den],
                 "first_sample": rows}
        print(f"{fx}: {says}\n\n| frame | first sample |\n| --- | --- |")
        for n, s in rows.items():
            print(f"| {n} | {s} |")
        print()

    for fx, (says, layer) in LAYER.items():
        rows = {str(f): heard(layer, f) for f in LAYER_FRAMES}
        c[fx] = {"says": says, "sample_rate": 48000, "frame_rate": [24, 1], "layer": layer,
                 "heard": rows}
        print(f"{fx}: {says}\n\n| composition frame | samples of the file heard |\n| --- | --- |")
        for f, r in rows.items():
            print(f"| {f} | {'silence' if r is None else f'{r[0]} up to {r[1]}'} |")
        print()

    files = wav_files()
    wavs = {}
    for name, raw in files.items():
        (OUT / "media" / name).write_bytes(raw)
        got = read_wav(raw)
        if "samples" in got:
            got["frames_at_24"] = length_frames(got["samples"], got["sample_rate"], 24, 1)
        wavs[name] = got
    c["FX-AUD-010"] = {"says": "What each WAV file in `Fixtures/audio/media/` is read as. "
                       "`frames_at_24` is how many frames of a 24 frame a second composition "
                       "the file covers.", "files": wavs}
    print("FX-AUD-010: " + c["FX-AUD-010"]["says"] + "\n\n| file | read as |\n| --- | --- |")
    for name, got in wavs.items():
        print(f"| `{name}` | {json.dumps(got)} |")
    print()

    (OUT / "media" / "bg.png").write_bytes(adj.png(adj.DRAWINGS["bg"]))
    (OUT / "media" / "half.png").write_bytes(adj.png(adj.DRAWINGS["half"]))
    for fx, sound in (("FX-AUD-020", True), ("FX-AUD-021", False)):
        name = fx.lower().replace("-", "_") + ".json"
        (OUT / name).write_text(json.dumps(project(fx, sound), indent=2) + "\n", encoding="utf-8")
    c["FX-AUD-020"] = {
        "says": "An audio layer between two picture layers changes no pixel: every frame of "
                "`fx_aud_020.json` is the picture of `fx_aud_021.json`, the same project "
                "without it, sample for sample, and neither has a warning.",
        "project": "fx_aud_020.json", "same_picture_as": "fx_aud_021.json", "frames": [0, 1, 2]}
    print("FX-AUD-020: " + c["FX-AUD-020"]["says"] + "\n")

    for fx, (says, change) in REFUSED.items():
        p = project(fx, True)
        change(p["compositions"][0]["layers"][1])
        name = fx.lower().replace("-", "_") + ".json"
        (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
        c[fx] = {"says": says, "project": name, "refused": "PROJECT_SCHEMA_INVALID"}
        print(f"{fx}: {says} Refused, `PROJECT_SCHEMA_INVALID`.\n")

    (OUT / "expected_audio.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")

    # The claims, worked by hand.
    t = c["FX-AUD-001"]["first_sample"]
    assert t["1"] == 2000 and t["86400"] == 172_800_000
    t = c["FX-AUD-002"]["first_sample"]
    assert [t[k] for k in "01234"] == [0, 1837, 3675, 5512, 7350] and t["24"] == 44100
    assert c["FX-AUD-003"]["first_sample"]["1000"] == 2_002_000
    t = c["FX-AUD-004"]["first_sample"]
    assert t["1"] == 1839 and t["24"] == 44144  # 24 * 1839.3375 = 44144.1
    h = c["FX-AUD-005"]["heard"]
    assert h["9"] is None and h["10"] == [0, 2000] and h["33"] == [46000, 48000]
    assert h["34"] is None and h["40"] is None
    assert c["FX-AUD-006"]["heard"]["10"] == [10000, 12000]
    h = c["FX-AUD-007"]["heard"]
    assert h["12"] is None and h["13"] == [0, 2000]
    h = c["FX-AUD-008"]["heard"]
    assert h["23"] == [46000, 47000] and h["24"] is None
    assert wavs["pcm16_mono_48k.wav"] == {"encoding": "pcm", "channels": 1, "sample_rate": 48000,
                                          "bits": 16, "samples": 4800, "frames_at_24": 3}
    assert wavs["pcm24_stereo_44k.wav"]["samples"] == 4410
    assert wavs["pcm24_stereo_44k.wav"]["frames_at_24"] == 3  # 2.4 frames: three hold a sample
    assert wavs["pcm8_mono_8k.wav"]["samples"] == 801  # odd length, padded, and the pad not counted
    assert wavs["extensible_6ch_48k.wav"]["encoding"] == "pcm"
    assert wavs["chunks_in_the_way.wav"]["samples"] == 480
    assert wavs["cut_short.wav"] == {"encoding": "pcm", "channels": 1, "sample_rate": 48000,
                                     "bits": 16, "samples": 480, "frames_at_24": 1,
                                     "warning": "MEDIA_AUDIO_CUT_SHORT"}
    assert wavs["no_sound.wav"]["samples"] == 0 and wavs["no_sound.wav"]["frames_at_24"] == 0
    assert wavs["adpcm.wav"] == {"encoding": "other", "channels": 1, "sample_rate": 22050}
    for name in ("rf64.wav", "no_fmt.wav", "no_data.wav", "not_a_wav.wav", "zero_rate.wav"):
        assert "refused" in wavs[name], name


if __name__ == "__main__":
    main()
