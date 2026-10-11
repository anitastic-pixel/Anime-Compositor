# B-349: Threshold RGB

Built on 2026-10-11 as D-469, under your /loop request and your go-ahead: **Threshold RGB**, in
**Stylize**, after CycoreFX's CC Threshold RGB (which comes with After Effects). Red, green and
blue are each turned to full or nothing at their own dividing line, so the picture becomes a
bold poster in at most eight pure colours. Threshold and Channel Threshold are unchanged.

- **Red Threshold**, **Green Threshold**, **Blue Threshold** (128 each, 0 to 255): each
  channel's dividing line. Lower turns more of that colour on.
- **Invert Red**, **Invert Green**, **Invert Blue** (Off): turn that one channel over.
- **Blend With Original** (0, 0 to 100): mixes the picture back in; 100 leaves it as it was.

The controls are CycoreFX's own, from its manual; the blend is our own rule, so it is not matched
to After Effects pixel for pixel.

The check, `verification/D-469_threshold_rgb_table.md` (221 of 221), holds every pixel to numbers
worked out by a separate program before the code existed, holds the graphics card's picture to
within 1 level of the processor's, and checks that Threshold RGB with three equal levels draws
exactly what Channel Threshold's RGB draws. The pictures are in `verification/D-469 pictures/`:
the street (`town.png`, `1_before.png` without the effect).

## What to check

1. **As added.** `2_as_added.png`: the street in pure colours (red, green, blue, yellow, cyan,
   magenta, black, white).
2. **More red.** `3_red_20.png`: Red Threshold 20: red switched on in more of the street, green and
   blue as in 2.
3. **Green turned over.** `4_invert_green.png`: 2 with only its green swapped, so green parts go
   black, red parts yellow, white parts magenta.
4. **Half blend.** `5_blend_50.png`: 2 mixed halfway back toward the street.
5. **In the app.** Add Threshold RGB (Stylize) to a picture; drag the three thresholds; flip the
   Inverts; raise Blend With Original.
6. **Out of range.** Type 256 in Red Threshold: it is refused with a sentence giving the range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-349_threshold_rgb_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Threshold RGB as added on each of three layers, played again, 61.8 ms a frame
on the card against 48.1 for the Noise alone; on the processor 192.9 against
94.6: a busy machine cannot say whether the card is quicker than the processor for this effect.
