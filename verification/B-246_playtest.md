# B-246: Color Offset

Built on 2026-10-08 under your effects loop request, decided as D-367. **Color Offset** (in
**Color Correction**) is our version of CycoreFX's CC Color Offset: each colour channel is turned
round by its own phase, for shifting, psychedelic colours. Key the phases to make them cycle.

The settings, with the values they start at:

- **Red, Green and Blue Phase** (0 each): degrees, -3600 to 3600; 360 is once round the range.
- **Overflow** (Wrap): what happens past white. Wrap comes round from black; Solarize folds back
  down; Polarize folds smoothly. With Solarize or Polarize 360 is the negative and 720 is back.

The check, `verification/D-367_color_offset_table.md` (114 of 114), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-367 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **Wrap.** `2_wrap_red_120.png`, red 120: the reds raised a third; the reddest colours come
   round to dark red.
2. **Solarize.** `3_solarize_all_180.png`, all three 180: darks lifted, lights folded down.
3. **Negative.** `4_polarize_all_360.png`, all three 360, Polarize: the street's negative.
4. **Psychedelic.** `5_polarize_mixed.png`, red 60, green -150, blue 240, Polarize: magenta and
   violet street.
5. **In the app.** Key the Red Phase from 0 to 720 over two seconds with Polarize: the colours
   cycle smoothly and come back.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Color Offset costs under 1 ms a 1080p layer, about 4 ms on the
processor (`verification/B-244_colour_timing_table.md`).

## Not built

- The phases turn the picture's sRGB values, where CycoreFX turns its stored 0 to 255 values.
- With Wrap, a whole turn (360) turns pure black to white and pure white to black, leaving the
  values between unchanged, as black and white are the same point on the circle.
