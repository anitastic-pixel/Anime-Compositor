# B-290: Heat Shimmer

Built on 2026-10-10 as D-411, under your /loop request: your plugin pick #17, Heat Shimmer, the
rippling haze above a hot road or a fire, drifting upward. PLUGINS.md suggested merging it into
Turbulent Displace as a drift "if anything" (it rated it useful but rare, since turbulence already
does most of it), so it is not a new effect: **Turbulent Displace** (in **Distort**) gains two
settings.

- **Drift Direction** (0): degrees, a dial, the way the ripple travels. 0 is up (rising heat),
  90 right, 180 down, 270 left.
- **Drift Speed** (0): pixels a frame, 0 to 1000, keyable. At 0 the ripple stays in place, as
  before. At 3 it slides 3 pixels each frame in the drift direction.

A Turbulent Displace in a project saved before this has no drift and draws exactly as it did.
For heat haze, use a small Amount (5 to 10), a Size of about 20, Drift Direction 0, Drift Speed 2
to 6, and a little Speed (evolution) so the ripple also changes as it rises. The plugin's own
blur and mask are not part of this: add Fast Box Blur after it, or a mask on the layer.

The check, `verification/D-411_heat_shimmer_table.md` (186 of 186), holds every pixel to numbers
worked out by a separate program before the code existed, checks that all older Turbulent
Displace test files (including the Line Boil ones) open, save and draw exactly as before, and
holds the graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-411 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **Rising.** `2_rising_frame_0.png`, `3_rising_frame_2.png` and `4_rising_frame_4.png` are
   frames 0, 2 and 4 with Amount 8, Size 20, Speed 0, Drift Direction 0 and Drift Speed 6. Flick
   through them: the same wobble pattern should slide upward, 12 pixels between each picture,
   without changing shape.
2. **Rising and changing.** `5_shimmer_speed_20_frame_4.png` is frame 4 of the same with Speed 20:
   the ripple rises and changes shape as it goes, the usual heat haze.
3. **In the app.** Add Turbulent Displace to a layer, set Amount 8, Size 20, Drift Speed 4; play:
   the ripple rises. Turn Drift Direction to 90: it travels right. Set Drift Speed to 0: it stops.
4. **An older project.** Open a project saved before today with a Turbulent Displace in it: Drift
   Direction and Drift Speed show 0 and the picture is the same as before.
5. **Out of range.** Type 1001 in Drift Speed: it is refused with a sentence saying it runs from
   0 to 1000, and the card keeps its old number.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-290_heat_shimmer_timing_table.md` (quiet machine): the reference shot with a moving Noise on three layers, played again,
16.9 ms a frame on the card drifting against 17.7 without, so the drift costs nothing
measurable; on the processor 124.2 against 124.4.
