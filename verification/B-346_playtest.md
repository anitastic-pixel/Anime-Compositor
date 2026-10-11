# B-346: Strobe Light

Built on 2026-10-11 as D-466, under your /loop request and your go-ahead: **Strobe Light**, in
**Stylize**, after After Effects' Strobe Light. At regular moments, or at random ones, the layer
flashes: for a short while it is painted with a colour (or mixed with it), or it goes clear.

- **Strobe Color** (white): the colour of the flash.
- **Blend With Original** (0): 0 is the full flash; higher keeps more of the picture; 100 none.
- **Strobe Duration** (0.1 seconds): how long each flash lasts.
- **Strobe Period** (1 second): from the start of one flash to the start of the next. A period
  no longer than the duration flashes all the time.
- **Random Strobe Probability** (0): the chance, per frame, of an extra flash at random.
- **Strobe** (Operates On Color Only): or Makes Layer Transparent, which fades the layer instead.
- **Strobe Operator** (Copy): how the colour is used: Copy, Add, Subtract, Multiply, Difference,
  And, Or, Xor, Minimum, Maximum, Darken, Lighten.
- **Random Seed** (0): a different set of random flashes.

The control names are After Effects' own (Adobe's page could not be read, so they come from the
After Effects page in Premiere 6.0's help); the operators past Copy and how the random flashes are
drawn are our own rule, so it is not matched to After Effects pixel for pixel.

The check, `verification/D-466_strobe_light_table.md` (214 of 214), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-466 pictures/`, the
street (`town.png`) filling the frame (`1_before.png` without the effect).

## What to check

1. **A flash.** `2_as_added_frame_0.png`: frame 0, inside the first flash: the whole frame white.
2. **Between flashes.** `3_as_added_frame_3.png`: frame 3, after the 0.1-second flash: the street
   as it was.
3. **Difference.** `4_difference_purple.png`: Difference with purple: the street's colours turned
   over against purple, like a negative tinted purple.
4. **Half red.** `5_red_blend_50.png`: Copy in red at Blend With Original 50: the street halfway
   to red.
5. **Clear.** `6_transparent_blend_25.png`: Makes Layer Transparent at Blend With Original 25: the
   street faint, a quarter strength.
6. **In the app.** Add Strobe Light (Stylize) to a picture and play: a white flash once a second.
   Shorten Strobe Period to 0.25 for a fast flicker; set Random Strobe Probability to 30 for
   random extra flashes; try the operators and Makes Layer Transparent.
7. **Out of range.** Type 101 in Blend With Original: it is refused with a sentence giving the
   range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-346_strobe_light_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Strobe Light flashing all the time on each of three layers, played again,
64.4 ms a frame on the card against 61.6 for the Noise alone; on the processor
136.0 against 98.7: a busy machine cannot say whether the card is quicker than the processor for this effect.
