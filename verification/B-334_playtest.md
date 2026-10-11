# B-334: Match Grain

Built on 2026-10-10 as D-454, under your /loop request: After Effects' Match Grain, in
**Generate** (beside Add Grain). It looks at another layer, works out how grainy it is in red,
green and blue, and adds grain that strong to this layer, so a clean shot can be made to sit
with grainy footage.

- **Noise Source Layer** (None): the layer whose grain is copied. It can be hidden. None leaves
  this layer as it is.
- **Intensity** (1): times the grain measured; 1 matches it, 2 doubles it, 0 adds none.
- **Red, Green and Blue Intensity** (1): the same, for each colour alone.
- Every other setting is Add Grain's, and does what it does there: Size, Softness, Aspect Ratio,
  Monochromatic, Saturation, Blending Mode, Shadows, Midtones, Highlights, Midpoint, Animation
  Speed, Animate Smoothly, Random Seed.

After Effects publishes no formula, ranges or starting values for this effect, so how the grain
is measured and laid on is our own rule, written down in document 21. The measure is a published
one (Immerkaer, 1996). After Effects' sample boxes, Compensate for Existing Noise, the preview
views, Tint and Channel Balance are not offered; ours always measures the whole source layer.

The check, `verification/D-454_match_grain_table.md` (152 of 152), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-454 pictures/`.

## What to check

1. **No source.** `2_no_source.png` is the same as `1_before.png`.
2. **Light grain matched.** `3_light_plate.png`: fine grain over the street, as on a hidden copy
   of the street with light Add Grain on it.
3. **Heavy grain matched.** `4_heavy_plate.png`: clearly heavier grain than 3, in every colour.
4. **In the app.** Import a grainy clip (or put Add Grain or Noise on a copy of a picture), and put
   Match Grain (Generate) on a clean layer. Pick the grainy layer as Noise Source Layer: grain
   appears, about as strong as the source's. Raise the source's grain: this layer's follows. Hide
   the source: it still works. Set Intensity to 0: the grain goes.
5. **Missing source.** Delete the source layer: this layer goes back to clean, and a warning names
   the missing layer.
6. **Out of range.** Type 11 in Intensity: it is refused with a sentence saying the range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge (After Effects says nothing on them): the whole source
layer is measured, not boxes you draw; Size and Softness are set by hand, not measured; with
Film (as added) the grain is strongest in each colour's middle tones, as Add Grain's.

## Speed

Measured only provisionally: the graphics card was 97% busy with other work while no cargo, rustc or test process was running
(`verification/B-334_match_grain_timing_table.md`). Three Match Grain on a 1920 by 1080 shot, card / processor,
milliseconds a frame played again: Noise alone 16.8 / 42.4 ms a frame, as added (no noise source) 16.8 / 43.2, reading layer2 111.8 / 115.8, reading layer2, size 3, softness 0.5, monochromatic 106.1 / 110.4; within Target P4 (100 ms or less on the processor): about 23 to 24 ms a layer on the processor, measuring included; on the card about 30 to 32 ms a layer, the measure being on the processor; to be measured again on a quiet machine.
