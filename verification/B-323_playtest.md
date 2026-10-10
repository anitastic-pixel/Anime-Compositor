# B-323: Add Grain

Built on 2026-10-10 as D-443, under your /loop request: After Effects' Add Grain, in
**Generate** (beside Noise; the app has no Noise & Grain group). It lays film grain over the
layer, a new grain every frame.

- **Intensity** (1): how strong the grain is; 0 turns it off.
- **Size** (1): how big each grain is, in pixels. **Softness** (0): 0 gives square grains, 1
  blends them smoothly into each other. **Aspect Ratio** (1): above 1 the grains are wider than
  tall.
- **Red, Green and Blue Intensity** (1 each): how strong the grain is in each colour.
- **Monochromatic** (Off): On gives a grey grain, the three colours moving together.
  **Saturation** (1): how coloured the grain is when it is not monochromatic; 0 is grey.
- **Blending Mode** (Film): *Film* puts the most grain in the middle tones and none in pure black
  or white; *Add* adds it straight on; *Overlay* lays it on as an Overlay layer.
- **Shadows, Midtones, Highlights** (1 each) and **Midpoint** (0.5): how much grain lands in the
  dark, middle and bright parts of the picture, and where the middle is.
- **Animation Speed** (1): how fast the grain changes; 0 holds one grain. **Animate Smoothly**
  (On): the grain flows from one to the next; Off makes it jump. **Random Seed** (0): a different
  grain.

After Effects publishes no formula or starting values for this effect, so how each pixel is
worked out is our own rule, written down in document 21. Not offered: After Effects' film-stock
presets, the preview box (View Mode, Preview Region), Channel Size, Tint, Channel Balance, its
other blending modes, and the matte (Match/Mask) settings. Blend with Original is the Mix every
effect has. The grain belongs to the drawing, so it moves with the layer. A draft draws at half
size with grains half the size, so it looks the same.

The check, `verification/D-443_addgrain_table.md` (264 of 264), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-443 pictures/`
(`1_before.png` is the street with nothing on it).

## What to check

1. **As added.** `2_as_added.png`: the whole street covered in a fine colour speckle, one pixel
   across, most visible in the sky and the coloured walls, the street's look otherwise kept.
2. **Coarse and grey.** `3_coarse_mono.png`: a bigger, softer speckle, grey rather than coloured,
   clearly stronger than picture 2.
3. **Into the shadows.** `4_shadows_overlay.png`: a heavy grain on the dark road and the windows,
   weaker in the bright sky.
4. **It moves.** `5_frame_24.png` against `2_as_added.png`: the same kind of speckle, but flip
   between them and every grain has changed.
5. **In the app.** Add Add Grain (Generate) to a layer and play: the grain fizzes. Set Animation
   Speed to 0: it holds still. Set Size 5 and Softness 1: big soft blotches; Softness 0: big
   squares. Turn Monochromatic on: grey. Set Highlights to 0: no grain in the brightest parts. Try
   Add and Overlay. Move the layer: the grain moves with it. Switch to Draft: the same look.
6. **Out of range.** Type 11 in Intensity: it is refused with a sentence saying it runs from 0
   to 10.
7. **Saved and opened again.** Save, close and open the project: the same settings, keys and
   picture.

## Speed

Measured on a quiet machine
(`verification/B-323_addgrain_timing_table.md`). Three Add Grains on a 1920 by 1080 shot, card / processor, milliseconds a
frame played again: Noise alone 12.3 / 41.7 ms a frame, as added 16.6 / 62.4, size 4, softness 1 18.1 / 67.8, size 3, softness 0.5, Overlay 18.7 / 75.5.
