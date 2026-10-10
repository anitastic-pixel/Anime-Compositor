# B-332: Noise HLS Auto

Built on 2026-10-10 as D-452, under your /loop request: After Effects' Noise HLS Auto, in
**Generate** (beside Noise HLS). It is Noise HLS (B-331) whose flecks of colour move by
themselves as the shot plays, with nothing to key.

- **Noise** (Uniform), **Hue** (0%), **Lightness** (10%), **Saturation** (0%) and **Grain Size**
  (1) work as in Noise HLS.
- **Noise Animation Speed** (1): how fast the flecks change. At 1 every frame has a whole new
  pattern; at 0.5 a new one every other frame, gliding between; at 0 they hold still.

After Effects publishes no formula or starting values for this effect, so how each pixel is
worked out is our own rule, written down in document 21. Every one of After Effects' settings is
offered.

The check, `verification/D-452_noisehlsauto_table.md` (187 of 187), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-452 pictures/`
(`1_before.png` is the street with nothing on it).

## What to check

1. **As added.** `2_as_added.png` and `3_as_added_frame_1.png`: the street finely flecked
   lighter and darker, a different pattern in each though nothing is keyed.
2. **Hue.** `4_hue_60_frame_5.png`: the colours flecked far round the wheel, the grey road
   staying grey.
3. **Grain.** `5_grain_4_saturation_60_frame_24.png`: soft blotches of stronger and weaker
   colour, a few pixels across.
4. **In the app.** Put Noise HLS Auto (Generate) on a colourful layer and play: the flecks
   shimmer frame by frame. Set Noise Animation Speed to 0.2: they drift slowly. Set it to 0: they
   stop. Choose Grain and raise Grain Size: soft moving blotches. Move the layer: the flecks move
   with it.
5. **Out of range.** Type 11 in Noise Animation Speed: it is refused with a sentence saying it
   runs from 0 to 10.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge (After Effects says nothing on them): at speed 1 every
frame is a new pattern; the speed runs from 0 to 10; the same choices as Noise HLS otherwise.

## Speed

Measured only provisionally: other lanes' builds and tests were running at the time
(`verification/B-332_noisehlsauto_timing_table.md`). Three Noise HLS Autos on a 1920 by 1080 shot, card / processor,
milliseconds a frame played again: Noise alone took 15.8 ms against 12.9 in B-331's quiet run): Noise alone 15.8 / 46.7 ms a frame, as added (Uniform, lightness 10, speed 1) 21.2 / 69.6, Squared, hue 40, lightness 20, saturation 40 24.5 / 68.8, Grain 2.5, hue 40, lightness 20, saturation 40 27.2 / 73.4; over Target P1: about 1.8 to 3.8 ms a layer on the card, against 1 ms (three value noises and an HSL round trip a pixel, in the grade pass's double precision, as Noise HLS); to be measured again.
