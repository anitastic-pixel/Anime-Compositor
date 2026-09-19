# ADR-018: Audio time is whole samples worked from whole frames, and the window's own decoder plays it

Status: PROPOSED
Date: 2026-09-19
Deciders: Andrew (owner)
Relates to: D-71 (reference audio), D-63 (audio deferred, now taken up), document 20 line 121, R-11 (offline), R-15

## Context

Document 20 says audio sample time is outside G1 and that adding it "requires an ADR and new
fixtures so the integer-frame contract is not retroactively reinterpreted". The owner asked for WAV
reference audio, and other audio files, on 2026-09-19. This is that ADR.

Two things have to be decided: how a frame number becomes a place in a sound file, and what
turns the file into sound.

## Decision

**1. Frames stay whole numbers and nothing about a picture changes.** An audio layer draws
nothing. The frame a picture shows is worked exactly as before; FX-AUD-020 pins that a project
with an audio layer renders the same samples as the project without it.

**2. A frame becomes a sample by whole-number arithmetic.** For a file of `rate` samples a second
in a composition of `numerator / denominator` frames a second, frame `n` of the file begins at
`floor(n * rate * denominator / numerator)`. No time is ever held in seconds as a float. Frame
`n` is the samples from there up to the first sample of frame `n + 1`, so at 44100 samples and 24
frames the frames take 1837 and 1838 samples in turn and none is dropped or played twice.

**3. The window's own decoder plays the sound.** The window is a WebView2 page, and it already
has a decoder and an output for WAV, FLAC, MP3, AAC in M4A, Ogg Vorbis and Opus. The build uses
that and adds no audio dependency: no decoder crate, no output crate, no new licence to review.
It works offline, because the file comes from the project's own folder through the scheme the
page already reads frames from.

**4. The core reads WAV headers only.** The core needs a file's length to draw its bar and to say
when a file is not what its name claims. It reads a WAV header by hand, which is about sixty
lines, and FX-AUD-010 pins it. It does not decode any other format: for those the length is what
the window's decoder reports.

## Consequences

- WAV and FLAC are exact to the sample. MP3 and AAC carry a short silence the encoder adds at the
  start, usually under 50 milliseconds, which is about one frame at 24 frames a second. The
  import dialog says so. For lip-sync work WAV is the format to use.
- Sound is a reference for timing. It is not in an exported PNG or EXR sequence, which has no
  place for it. Writing the mix beside a sequence is a later unit if it is asked for, and video
  with sound waits on D-30.
- The waveform on a layer's bar is drawn by the window from the decoded sound, the same way for
  every format, and is judged by eye in a playtest rather than by a fixture.
- If a format stops being played by a future WebView2, the layer stays in the project, is
  silent, and says why. Nothing is lost.

## Alternatives not taken

A decoder and output in the core (`symphonia` and `cpal`) would make every format exact and
testable by fixture, and would add some forty crates, an MPL-2.0 decoder, and a real-time audio
thread that document 33 says needs its own design. That is a large cost for reference sound.
It can replace decision 3 later without changing the file format or decisions 1, 2 and 4.
