# B-392: Channel Wipe

Built on 2026-10-11 as D-512, under your /loop request: Channel Wipe, in **Transition**, our own
take on CycoreFX's CC Image Wipe (a name of our own). The layer is wiped away in the order of a
chosen channel of another layer, or of itself: the lowest values go first. It is Gradient Wipe's
engine with more choices, and Gradient Wipe itself draws exactly as before.

- **Gradient Layer** (none): the layer whose channel decides the order; with none, the layer
  itself.
- **Fit** (Stretch): how a gradient layer of another size is fitted: Stretch, Center or Tile.
- **Completion** (0): how far the wipe has gone; 0 the layer whole, 100 all gone.
- **Border Softness** (10): how soft the wiping edge is.
- **Property** (Luminance): which channel is read: Red, Green, Blue, Alpha, Luminance,
  Lightness, Hue or Saturation.
- **Blur** (0): blurs the gradient first, so the wipe follows broad shapes, not every detail.
- **Inverse Gradient** (off): the highest values go first instead.

CycoreFX also has Auto Softness (softness that follows how fast Completion moves); it is not
built, because an effect only sees its settings at its own frame.

CycoreFX publishes no formula and no settings when added, so the rule and those settings are
ours, written down in document 21.

The check, `verification/D-512_channel_wipe_table.md` (353 of 353), holds every
pixel to numbers worked out by a separate program before the code existed, holds the graphics
card's picture to within 1 level of the processor's, and runs Gradient Wipe's own checks again
unchanged. The pictures are in `verification/D-512 pictures/`; the checkerboard or white you see
in a viewer is where the layer is gone.

## What to check

1. **The street.** `1_street.png`: the shot without the effect. `2_reveal.png` is the street in
   its opposite colours, used as another layer in picture 6.
2. **Its own luminance, half way.** `3_itself_luminance_half.png`: the dark parts (the road,
   the dark windows) gone, the light parts exactly as they were.
3. **Its own hue.** `4_itself_hue_soft.png`: the same read in hue: what comes earliest round the
   colour wheel goes first (the tan and green buildings, and the white and grey parts, which
   read as hue 0); the blue buildings and the sky stay.
4. **Blurred.** `5_itself_luminance_blur.png`: luminance with Blur 20: the gone parts follow broad
   rounded shapes rather than every window.
5. **Another layer.** `6_other_layer.png`: the opposite-colours street as the Gradient Layer, so
   the light parts of the street go first; the table shows it is exactly Gradient Wipe's picture
   with the same settings.
6. **In the app.** Add Channel Wipe (Transition) to a layer over another, keyframe Completion
   from 0 to 100 and play. Try each Property, a Gradient Layer, Blur and Inverse Gradient.
7. **Gradient Wipe unchanged.** Open a project with a Gradient Wipe: it looks as it did.
8. **Out of range.** Type 120 in Blur: it is refused with a sentence saying the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; how each property is read;
that Auto Softness is missing.

## Speed

`verification/B-392_channel_wipe_timing_table.md`. Three Channel Wipes on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL (machine not quiet, other lanes' tests on the card), card 72.4 and 268.8 ms a frame (the layer itself; layer 4 in hue, blur 6) against 91.0 for the Noise alone, processor 180.8 and 354.3 against 109.0; a quiet re-measure is owed.
