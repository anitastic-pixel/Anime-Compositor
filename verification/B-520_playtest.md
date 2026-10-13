# B-520: sound in an export (the third step of the sound plan)

Built on 2026-10-13 as the third step (S-3) of the sound plan you accepted (D-638), decided as
D-641. Until now an export had no sound. Now an MP4 carries the composition's sound, and a PNG
or EXR sequence or an animated PNG can write its sound as a WAV file beside the frames.

The check, `verification/D-641_sound_export_table.md` (65 of 65 checks pass), exports eleven
small test projects. Each MP4 test puts a single click on chosen frames, and two different
players (ffmpeg and Windows' own) must find each click exactly on that frame's first sample.
The **click table** at the bottom lists every one: all land.

## What to check

**The film.** Open `verification/D-641 film/flash_and_click.mp4` in Windows' Media Player
(or Films & TV). It is three seconds long: the screen flashes white five times, and each flash
has a click.

1. Each click is heard **with** its flash, not before or after it.
2. Nothing else is heard: no hiss or pop at the start or end.

**In the app.**

3. The format list beside Export now says **"MP4 (H.264), over black, with sound"** and
   **"GIF, a preview: 256 colours, no sound"**.
4. Choose **PNG sequence**: a box **"Sound as a WAV beside it"** appears. Choose GIF or MP4:
   the box hides.
5. In a composition with a sound layer, tick the box and export a PNG sequence. A `.wav` file
   named like the frames (for `shot_%04d.png`, `shot.wav`) appears in the same folder, and it
   plays the composition's sound.
6. Export the same composition as an MP4 and play it: the sound is in it, in step with the
   pictures.
7. Turn a sound layer up to +12 dB and export: a warning says the sound goes past full level
   and in how many places. The file is still written.
8. **Older projects.** Open a project that already has sound in it: it plays, draws and saves
   exactly as before. An MP4 of a composition with no sound layer is the same silent film as
   before.

## Not built (known limits)

- On a computer that is not Windows, an MP4 is still written without sound, with a warning
  saying so; the WAV beside a sequence works everywhere.
- Some players that ignore the MP4's end marker (ffmpeg among them) play about a hundredth of a
  second of silence past the end. Windows' player stops exactly on time.
- Solo does not change what an export writes, sound or picture, as before.
- Sound inside a composition layer (a precomposition), keyed level and pan come in S-7.
