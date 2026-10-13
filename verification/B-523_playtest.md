# B-523: Distortion (the sixth step of the sound plan)

Built on 2026-10-13 as the sixth step (S-6) of the sound plan you accepted (D-638), decided as
D-644. Distortion is the third sound effect: it deliberately overdrives a sound, from a warm
thickening to a harsh fuzz, and can also make it grainy and lo-fi. Like Noise Gate and
Compressor, it goes on sound layers only.

The check, `verification/D-644_distortion_table.md` (240 of 240 checks pass), runs eighteen small
test sounds through it and holds every result to numbers worked out before the code.

## What to look at

**The curves** in `verification/D-644 pictures/`, the files ending `_curve`. Across is the
sound going in (-1 to +1), up is what comes out; the thin grey diagonal is "unchanged".

1. `fx_dist_001_curve` (Soft Clip): an S shape that flattens smoothly as it nears full level.
2. `fx_dist_002_curve` (Hard Clip): steep and straight, then cut off flat at full level, with
   sharp corners.
3. `fx_dist_003_curve` and `fx_dist_004_curve` (Saturation 1 and 2): gentler S shapes that never
   quite flatten.
4. `fx_dist_005_curve` (Tube): lopsided; the top flattens lower than the bottom. This is what
   gives Tube its different, "warmer" colour.
5. `fx_dist_006_curve` (Fuzz): rises very sharply out of the middle, then flat.

**The sine close-ups** (`_sine`: two cycles, distorted above, clean below): the clean wave is
round; Hard Clip and Fuzz flatten its tops into near squares, Soft Clip rounds them.

**The waveforms** (left channel above, right below; `_before` with the effect off):

6. `fx_dist_009`: Resolution 4 bits: the ramp becomes a staircase.
7. `fx_dist_011`: Drive keyed from 0 to 100%: the sine grows fatter and squarer as it goes.
8. `fx_dist_012`: Downsample keyed from 1 to 32: the ramp becomes coarser steps.

**The harmonic table** in the check: every type except Tube adds only odd harmonics (no 2nd); Tube
adds a 2nd at -20.5 dB. Fuzz is the harshest (3rd at -12.6 dB), Soft Clip the mildest.

## What to check in the app

9. Select a sound layer. The Effects list offers **Audio: Compressor**, **Audio: Distortion** and
   **Audio: Noise Gate**; a picture layer is offered none of them.
10. Add Distortion to a voice or some music and play: with the starting values (Soft Clip, Drive
    25%) it sounds louder and thicker.
11. Try each **Type**: Hard Clip and Fuzz sound harsh, Saturation and Soft Clip warmer, Tube
    slightly different in character from Soft Clip.
12. Turn **Drive** up to 100%: much harsher. Turn **Volume** down if it is too loud (11 is
    "as it is").
13. Set **Mix** to 30%: mostly the clean sound with some grit under it.
14. Set **Resolution** to 6 bits, then **Downsample** to 8: a crunchy, old-video-game sound.
15. Key **Drive** (the stopwatch) from 0 to 100% over a few seconds and play: it grows smoothly,
    with no clicks where the keys change.
16. The card has no picture **Mix** row (its own Mix is a sound setting). Export the composition
    as an MP4: it sounds as in the window.
17. **Older projects.** Open a project with sound in it: it plays, draws and saves as before.

## Not built (known limits)

- No oversampling: at high Drive, especially with Hard Clip or Fuzz on high notes, a faint
  metallic edge (aliasing) can be heard. Logged; it can be added if you hear it.
- The six curves are ours, so the sound will not match After Effects' exactly at the same
  settings.
- Keyed level, pan and sound through composition layers follow as S-7.
