# B-348: Channel Threshold

Built on 2026-10-11 as D-468, under your /loop request and your go-ahead: **Channel Threshold**,
in **Stylize**, after CycoreFX's CC Threshold (which comes with After Effects). It is Threshold
with more choices: every pixel turned to full or nothing by its brightness, its colours, its
saturation or its covering. Threshold itself is unchanged.

- **Threshold** (128, 0 to 255): the dividing line. Lower turns more of the picture white.
- **Channel** (Luminance): Luminance makes it black and white by brightness, exactly as
  Threshold does; RGB works red, green and blue on their own, so the picture keeps eight pure
  colours; Saturation turns the vivid parts white and the greys black; Alpha works only the
  covering, so soft edges turn hard and the colours stay.
- **Invert** (Off): turns the result over, white for black.
- **Blend With Original** (0, 0 to 100): mixes the picture back in; 100 leaves it as it was.

The controls are CycoreFX's own, from its manual; how brightness and saturation are read, and the
blend, are our own rule, so it is not matched to After Effects pixel for pixel.

The check, `verification/D-468_channel_threshold_table.md` (206 of 206), holds every pixel to
numbers worked out by a separate program before the code existed, holds the graphics card's
picture to within 1 level of the processor's, and checks that Channel Threshold on Luminance
draws exactly what Threshold draws. The pictures are in `verification/D-468 pictures/`: the
street (`town.png`, `1_before.png` without the effect).

## What to check

1. **As added.** `2_as_added.png`: the street in pure black and white, as Threshold at 128.
2. **RGB.** `3_rgb.png`: the street in pure colours (red, green, blue, yellow, cyan, magenta,
   black, white).
3. **Saturation.** `4_saturation.png`: black and white again, but the colourful parts white and
   every grey part black.
4. **Turned over.** `5_turned_over.png`: 2 with white and black swapped.
5. **Half blend.** `6_blend_50.png`: 2 mixed halfway back toward the street.
6. **In the app.** Add Channel Threshold (Stylize) to a picture; drag Threshold; try each
   Channel; turn on Invert; raise Blend With Original.
7. **Out of range.** Type 256 in Threshold: it is refused with a sentence giving the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-348_channel_threshold_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a
moving Noise and a Channel Threshold as added on each of three layers, played again,
47.0 ms a frame on the card against 29.5 for the Noise alone; on the processor
98.7 against 123.4: a busy machine cannot say whether the card is quicker than the processor for this effect.
