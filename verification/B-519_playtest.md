# B-519: hearing sound in step (the second step of the sound plan)

Built on 2026-10-13 as the second step (S-2) of the sound plan you accepted (D-638), decided as
D-640. Until now the window played each sound file itself. Now the program mixes the
composition's sound (every sound layer, at its level, from the right sample of each frame) and
the window plays what the program mixed, a second at a time. This is the same mix the export
will write in the next step, so what you hear is what you will get.

The check, `verification/D-640_sound_mix_table.md` (72 of 72 checks pass), mixes fourteen small
test projects and compares every sample with the numbers worked out before the code. Its
**level table** (at the bottom) lists how loud each mix is: for example a tone at -6 dB on a
layer turned down 6 dB comes out at -12.00 dB, and a layer turned up 12 dB is not cut off.

## What to check

**The pictures** in `verification/D-640 pictures/` show each test mix: the left channel above,
the right below, and a thin grey line where each frame starts.

1. **`fx_smix_004.png`**: the sound starts exactly on the sixth grey line (frame 5) and is
   silent before it.
2. **`fx_smix_006.png`**: the sound stops exactly on the eleventh grey line (frame 10).
3. **`fx_smix_002.png`**: empty (the layer is switched off).

**In the app: the clicks land on their drawings.** The test sound
`Fixtures/sound_mix/media/clicks.wav` is silent except for two short clicks, exactly at the
start of frame 2 and frame 12 at 24 frames a second.

4. Open the reference shot (24 frames a second). Import `clicks.wav` and make a layer of it
   starting at frame 0.
5. **Step.** Go to frame 1, then press the next-frame key: you hear a click as frame 2 shows
   (layer 3, the one on 2s, changes drawing there). Step on to frame 3: silence. Step from 11
   to 12: a click again. No other step clicks.
6. **Play** from frame 0: two clicks in the first half second, the second as frame 12 shows.
7. **Level while playing.** Play a longer sound (any song), and while it plays turn its layer's
   level down to -20: it gets quieter within about a second, without stopping.
8. **Solo.** Solo a picture layer while a sound layer exists: the sound goes quiet, as before.
9. **Opus.** An `.opus` sound still plays in the window, as before (the program does not read
   it, so it plays from the file).
10. **Older projects.** Open a project that already has sound in it: it plays, draws and saves
    exactly as before.

## Not built (known limits)

- Sound inside a composition layer (a precomposition) is not heard yet, and level cannot be
  keyed or panned yet: that is S-7 of the plan.
- Stretching a sound layer, or time remapping it, does not change its sound, as before.
- The very first time a long sound is played after opening, the program works out its 48,000
  a second copy, which can take a fraction of a second; the sound then starts a little way in
  rather than late, so it stays on its frames.
