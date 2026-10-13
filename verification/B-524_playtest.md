# B-524: keyed level, pan, and sound through composition layers (the seventh step of the sound plan)

Built on 2026-10-13 as the seventh and last step (S-7) of the sound plan you accepted (D-638),
decided as D-645. Three things change:

- A sound layer's **level** can be keyed, so music can fade in and out under dialogue.
- A sound layer has a **pan**: -100 is all left, +100 all right, 0 (or none) the middle.
- Sound inside a **composition layer** is now heard, where that layer plays it in time.

The check, `verification/D-645_sound_layer_table.md` (126 of 126 checks pass), runs eighteen
small test projects and holds every result to numbers worked out before the code. Its test
clicks are single sharp ticks twice a second, so where a tick lands says exactly where the sound
landed.

## What to look at

The pictures in `verification/D-645 pictures/` are waveforms: the left channel above, the right
below.

1. `fx_slay_001`: a tone whose level is keyed from -24 dB to 0 dB: it swells smoothly, no steps.
2. `fx_slay_002`: a level keyed down to silence after a Noise Gate: it fades smoothly out.
3. `fx_slay_004`: pan +50: the top (left) half the height of the bottom (right).
4. `fx_slay_005`: pan keyed from all left to all right: the top fades out as the bottom fades in.
5. `fx_slay_008`: a composition layer starting later: its ticks start later by the same amount.
6. `fx_slay_010`: the inner composition is short: the sound stops where it ends.
7. `fx_slay_011`: two compositions deep: the ticks are where both offsets put them, quieter.
8. `fx_slay_013`: a 30 fps composition inside a 24 fps one: some ticks doubled (a known limit).
9. `fx_slay_017`: one composition used twice: each tick heard twice.

## What to check in the app

10. **Older projects.** Open a project with sound in it: it plays, draws and saves as before.
11. **Composition layers.** Put a sound layer inside a composition, then use that composition
    as a layer in another one. Play the outer one: you hear the sound, starting where the
    composition layer starts. Slide the layer later: the sound moves with it. Switch the layer
    off: the sound stops too.
12. Export the outer composition as an MP4: it has the sound, in the same place as in the window.
13. **Level.** Select a sound layer: the panel's level number still sets the level as before.
14. **Keys and pan from a file.** Open `Fixtures/sound_layer/fx_slay_001.json`: the panel shows
    "Level keys" and the sound swells over the first second. Open `fx_slay_006.json`: the panel
    shows "Pan: 30 left", and the sound is to the left in headphones.

## Not built (known limits)

- A composition layer has no level or pan of its own; set them on the sound layers inside.
- A composition at a different frame rate, stretched, or time-remapped is heard frame by
  frame, so short pieces can repeat or be skipped instead of being sped up or slowed down.
- One switch mutes a composition layer's picture and sound together.
- The panel shows level keys and pan but cannot add level keys or set the pan yet; the level
  number sets the keys' starting value.
- An Opus sound file inside a composition inside another is not heard in the window (an
  export has it).
- Audio Spectrum and Audio Waveform read the level's starting value, not its keyed value.
