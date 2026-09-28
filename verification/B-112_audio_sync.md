# B-112: sound in other formats, measured rather than built

Written on 2026-09-27 against D-172, which is **proposed** and waits for the owner.

## The question

B-112 asked for MP3, FLAC, Ogg Vorbis and AAC to be "read as the same samples a WAV gives". D-71
had left them to the window's own decoder. It warned that MP3 and AAC "can start up to about a
frame late", which would matter for lip sync.

Before building a decoder into the core, I measured whether that warning is true.

## How it was measured

`tools/audio_sync_probe.py` does four things:

1. It writes one second of silence at 48 kHz, with a click on the first sample of frame 2 and one
   on the first sample of frame 12, at 24 frames a second. That is sample 4000 and sample 24000.
2. It encodes that file into each format with ffmpeg.
3. It decodes each one back with ffmpeg.
4. It writes `decode.html`, which asks a browser's own decoder the same question. The window's
   web view uses that same decoder, Chromium's.

Everything is in `verification/B-112 probe/`.

## What came back

| File | ffmpeg 8.1: samples, clicks at | Chrome 153: samples, clicks at | On the frame |
|---|---|---|---|
| clicks.wav | 48000; 4000, 24000 | (the window reads WAV itself) | yes |
| clicks.flac | 48000; 4000, 24000 | 48000; 4000, 24000 | yes |
| clicks.mp3 | 48000; 4000, 24000 | 48000; 4000, 24000 | yes |
| clicks.ogg (Vorbis) | 47872; 4000, 24000 | 47872; 4000, 24000 | yes |
| clicks.m4a (AAC) | 48128; 4000, 24000 | 48000; 4000, 24000 | yes |
| clicks.opus | 48000; 4000, 24000 | 48000; 4000, 24000 | yes |
| clicks.noheader.mp3 | 49536; 5105, 25105 | 49536; 5105, 25105 | **no**, 1105 samples late |

**Every format starts on exactly the sample the WAV does, in both decoders.** The two decoders
also agree with each other to the sample.

The only differences are at the very end of the file:

- Ogg Vorbis comes back 128 samples short.
- AAC comes back 128 samples long in ffmpeg.

128 samples is 1/15 of a frame, in the silence after the last sound. It changes nothing that is
heard.

The last row is the exception. An MP3 written without its gapless header (the "Xing" or "LAME"
tag, which almost every encoder writes) starts 1105 samples late, a little over half a frame. It
is late in both decoders, because the delay is in the file itself. No decoder, ours included,
could know how much to trim.

## What this means

- The window already plays these formats in step with the frames, to the sample.
- It already tells you when it cannot play a file. The status line says the file "could not be
  played, so its layer is silent".

So B-112 as written would add a decoder library to the core, with more third-party licences and a
patent question over AAC. Its only job would be to hand back samples the window already has.
Nothing in the core would use them until sound goes into exports, which D-71 keeps out.

D-172 proposes closing B-112 on this evidence. It also corrects D-71's "up to about a frame late"
to what was measured.

## To check it yourself in the window

1. Open any project and **Import** `verification/B-112 probe/clicks.mp3`. Press **Make a layer**.
2. Zoom the timeline in until single frames are wide. The waveform on the green bar has two
   spikes. Each starts at the left edge of a frame: frames 2 and 12 of the layer.
3. Step to frame 2 with the arrow key. You hear the click. Step to frame 1: silence.
4. Do the same with `clicks.flac`, `clicks.ogg`, `clicks.m4a` and `clicks.opus`. They behave
   identically.
5. Import `clicks.noheader.mp3`. Its spikes sit about half a frame to the right of the others.
   This is what an MP3 with no gapless header does everywhere.

## Result

Not yet played.
