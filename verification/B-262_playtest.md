# B-262: Levels with a set for each channel, and Levels (Individual Controls)

Built on 2026-10-09 under your effects loop request, decided as D-383 by your choice "Both names,
one engine". **Levels** (in **Color Correction**) now has a **Channel** menu: RGB, Red, Green,
Blue and Alpha, each with its own Input Black, Input White, Gamma, Output Black and Output White.
**Levels (Individual Controls)** is the same effect under a second name, with all 25 settings
shown at once.

How the sets combine: each of red, green and blue goes through its own set first, then through
the RGB set. The Alpha set changes the covering, the colours kept. The Channel menu only picks
which set the controls show; it never changes the picture.

Levels saved before this change opens as the RGB set and draws exactly as before.

All 25 settings can be keyed. The check, `verification/D-383_levels_individual_table.md` (146 of
146), holds every pixel to numbers worked out by a separate program before the code existed, and
the graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-383 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** Add Levels (Individual Controls) to a drawing: nothing changes.
2. **Old Levels unchanged.** `2_levels_as_before.png` is a Levels saved the old way (input 20 to
   230, gamma 1.2, output 10 to 245). It is the same street as before this change. Open an old
   project with Levels: it should look exactly as it did.
3. **Warmer.** `3_warm.png`: Red Input White 220 and Blue Output White 210. The street is warmer,
   and green is untouched.
4. **The Channel menu.** `4_channel_blue_gamma.png`: Levels with the menu on Blue and Blue Gamma
   1.6. The middles are bluer. In the app, switch the menu between RGB, Red, Green, Blue and
   Alpha. The controls should change to that set's values, and the picture should not change.
5. **Alpha.** `5_alpha_half.png`: Alpha Output White 128. The street is see-through by half
   everywhere, with its colours kept.
6. **In the app.** Key Red Gamma from 1 to 3 over a second. The reds in the middle tones should
   grow brighter.
7. **Saved and opened again.** Change a few sets, save, close and open again. You should get the
   same settings and the same picture.

**Timing.** PROVISIONAL, the machine was busy (`verification/B-262_levels_timing_table.md`): the reference shot with a moving Noise on three layers, played again. A Levels saved before costs the same as it did, within the noise; all four colour sets at once add under 1 ms a layer on the graphics card.

## Not built

- **The histogram.** After Effects shows one behind the controls; ours draws the ramps only.
- **The "Clip To Output Black/White" switches.** After Effects shows these for 32-bit colour.
  Ours always clips.
- **Keying the Levels as a whole.** After Effects keys Levels as one property. Here, each
  setting has its own keys.
- **Card use for some settings.** An Alpha set that changes anything, or a set whose input black
  equals its input white (a threshold), is drawn by the processor. The rest of the effect is on
  the graphics card.
- **The order of the sets is ours.** Adobe does not publish the order its channels are applied
  in. We use each channel's own set, then RGB, the same order as Curves.
