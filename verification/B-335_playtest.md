# B-335: Remove Grain

Built on 2026-10-10 as D-455, under your /loop request: After Effects' Remove Grain, in
**Blur & Sharpen** (beside Dust & Scratches). It works out how grainy the layer is in red, green
and blue, then smooths the grain away while keeping edges hard, and can sharpen the result.

- **Noise Reduction** (1): how strongly the grain measured is smoothed, 0 to 3. 0 leaves the
  layer as it is.
- **Passes** (1): 1 to 4. Each pass smooths over a wider area than the last, for coarser grain.
- **Mode** (Multichannel): Multichannel judges the three colours together; Single Channel judges
  each colour alone (better when the grain is in one colour only).
- **Unsharp Mask Amount** (0), **Radius** (1) and **Threshold** (0): sharpen the result, as
  Sharpen does, to bring back edge detail the smoothing softened.

After Effects publishes no formula, ranges or starting values for this effect, so how the grain
is measured and removed is our own rule, written down in document 21, built from published
methods (Immerkaer 1996 for the measure, a bilateral filter for the smoothing). After Effects'
preview views and box, Channel Noise Reduction, Fine Tuning, Temporal Filtering and Sampling are
not offered; ours always works on the whole layer.

The check, `verification/D-455_remove_grain_table.md` (137 of 137), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-455 pictures/`.

## What to check

1. **The grain.** `2_grainy.png` is the street (`1_clean.png`) with coloured grain all over.
2. **As added.** `3_as_added.png`: most of the grain gone, the house edges, windows and road
   markings still hard.
3. **Strongest.** `4_strong.png` (Noise Reduction 3, three passes): smoother still, but the edges
   go soft. That is expected: so strong a setting on so heavy a grain smooths across edges too.
4. **Sharpened.** `5_strong_sharpened.png`: 4 with the Unsharp Mask at 150: the edges come back
   firmer than in 4.
5. **In the app.** Put Noise or Add Grain on a picture, then Remove Grain (Blur & Sharpen) under
   it: the grain mostly goes. Raise Noise Reduction: smoother. Set it to 0: the grain is back as
   it was. Try Single Channel and Passes 2.
6. **Out of range.** Type 4 in Noise Reduction: it is refused with a sentence saying the range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge (After Effects says nothing on them): how strong Noise
Reduction 1 is (we think it should remove most grain without softening edges); the whole layer is
measured, not a box you place.

## Speed

Measured only provisionally: the card was at 96 to 100 per cent with other work and lane A's b327 test was running
(`verification/B-335_remove_grain_timing_table.md`). Three Remove Grain on a 1920 by 1080 shot, card / processor,
milliseconds a frame played again: Noise alone 40.1 / 52.0 ms a frame, as added 196.0 / 183.9, Noise Reduction 2, three passes, single channel 325.8 / 480.8, as added with Unsharp Mask 150 436.9 / 217.9; over Target P3 (8 ms or less added on the card): about 52 to 132 ms a layer on the card, the measure (the layer drawn again up to the effect, then summed) on the processor included; on the processor about 44 to 143 ms a layer; to be measured again on a quiet machine.
