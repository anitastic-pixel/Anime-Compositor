"""B-112: does a sound in another format start on the same sample as its WAV?

Writes one WAV with two clicks on frame starts (frames 2 and 12 at 24 frames a second), encodes
it with ffmpeg to FLAC, MP3, Ogg Vorbis, AAC in M4A and Opus, and decodes each back with ffmpeg,
reporting the length and where each click landed. It also writes `decode.html`, which asks a
browser's own decoder (Chromium's, the one the window's web view uses) the same question.

Run from the repository root: python tools/audio_sync_probe.py
Needs ffmpeg and numpy. Output goes to `verification/B-112 probe/`.
"""
import base64
import json
import pathlib
import subprocess

import numpy as np

OUT = pathlib.Path("verification/B-112 probe")
RATE = 48000
CLICKS = [4000, 24000]  # frames 2 and 12 at 24 frames a second begin on these samples
FORMATS = {
    "flac": ["-c:a", "flac"],
    "mp3": ["-c:a", "libmp3lame", "-b:a", "192k"],
    "ogg": ["-c:a", "libvorbis", "-q:a", "5"],
    "m4a": ["-c:a", "aac", "-b:a", "192k"],
    "opus": ["-c:a", "libopus"],
    # The counter-case: an MP3 with no gapless header carries its encoder's delay in any decoder.
    "noheader.mp3": ["-c:a", "libmp3lame", "-b:a", "192k", "-write_xing", "0"],
}


def ffmpeg(*args):
    subprocess.run(["ffmpeg", "-v", "error", "-y", *args], check=True)


def clicks(samples):
    first = int(np.argmax(np.abs(samples[:12000])))
    second = 12000 + int(np.argmax(np.abs(samples[12000:36000])))
    return [first, second]


def main():
    OUT.mkdir(exist_ok=True)
    wav = OUT / "clicks.wav"
    beats = "+".join(f"eq(n\\,{c})" for c in CLICKS)
    ffmpeg("-f", "lavfi", "-i", f"aevalsrc='if({beats}\\,0.9\\,0)':s={RATE}:d=1", "-c:a", "pcm_s16le", str(wav))
    rows = []
    for ext, codec in {"wav": [], **FORMATS}.items():
        path = OUT / f"clicks.{ext}"
        if ext != "wav":
            ffmpeg("-i", str(wav), *codec, str(path))
        raw = subprocess.run(
            ["ffmpeg", "-v", "error", "-i", str(path), "-f", "f32le", "-ac", "1", "-"],
            check=True, capture_output=True,
        ).stdout
        samples = np.frombuffer(raw, dtype="<f4")
        rows.append((ext, len(samples), clicks(samples)))
    version = subprocess.run(["ffmpeg", "-version"], capture_output=True, text=True).stdout.split("\n")[0]
    table = ["| File | Samples | Clicks at | On the WAV's samples |", "|---|---|---|---|"]
    table += [f"| clicks.{e} | {n} | {c[0]}, {c[1]} | {'yes' if c == CLICKS else '**NO**'} |" for e, n, c in rows]
    (OUT / "ffmpeg_decode.md").write_text(
        f"Decoded by {version}. The WAV holds {RATE} samples with clicks at {CLICKS[0]} and "
        f"{CLICKS[1]}.\n\n" + "\n".join(table) + "\n", encoding="utf-8")
    files = {e: base64.b64encode((OUT / f"clicks.{e}").read_bytes()).decode() for e in FORMATS}
    (OUT / "decode.html").write_text(PAGE.replace("FILES", json.dumps(files)), encoding="utf-8")
    print("\n".join(table))


PAGE = """<!doctype html><meta charset="utf-8"><title>B-112 decode probe</title><body>working</body>
<script>
const D = FILES;
(async () => {
  const out = [navigator.userAgent];
  for (const [e, b] of Object.entries(D)) {
    try {
      const u = Uint8Array.from(atob(b), (c) => c.charCodeAt(0));
      const buf = await new OfflineAudioContext(1, 48000, 48000).decodeAudioData(u.buffer);
      const a = buf.getChannelData(0);
      let p = 0, q = 12000;
      for (let i = 0; i < 12000; i++) if (Math.abs(a[i]) > Math.abs(a[p])) p = i;
      for (let i = 12000; i < Math.min(a.length, 36000); i++) if (Math.abs(a[i]) > Math.abs(a[q])) q = i;
      out.push('clicks.' + e + ': ' + buf.length + ' samples, clicks at ' + p + ', ' + q);
    } catch (x) { out.push('clicks.' + e + ': not decoded, ' + x); }
  }
  document.body.innerText = out.join('\\n');
  document.title = 'done';
})();
</script>
"""

if __name__ == "__main__":
    main()
