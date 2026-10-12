# B-470: CC Force Motion Blur

Built on 2026-10-12 as D-581 (a), after you said "go with your recommendations on all six":
CC Force Motion Blur, under **Time**, our version of CycoreFX's effect of that name. Put it on a
composition layer (a precomposed layer) and it blurs whatever moves *inside* that composition,
by drawing the inside several times across the shutter and averaging the pictures.

- **Motion Blur Samples** (8): how many in-between pictures are averaged; more is smoother and
  slower.
- **Override Native Shutter** (Off): Off uses the composition's own shutter angle, and only when
  the composition's motion blur and the layer's motion blur switch are both on. On uses the
  Shutter Angle below whatever the switches say.
- **Shutter Angle** (180): how long the shutter stays open, used only with Override on; 360 is a
  whole frame.
- **Native Motion Blur** (On): On keeps the motion blur of layers inside the composition; Off
  draws them without it.

It works only on a composition layer without a Time Remap. On any other layer it leaves the
picture as it is and says so each frame (it does not quietly do nothing). Not built yet: on
adjustment layers, on a layer with a Time Remap, and blurring between frames of drawings, text,
shapes or expressions inside, which hold still for the whole frame. CycoreFX gives no formula,
so how the in-between pictures are taken and the settings when added are ours.

The check, `verification/D-581_force_motion_blur_table.md` (126 of 126), holds every pixel to
numbers worked out by a separate program before the code existed. The pictures are in
`verification/D-581 pictures/`: a white square moving left to right across a night-blue solid,
inside a composition layer, at frame 6.

## What to check

1. **Before.** `1_before.png`: the square sharp, no effect.
2. **As added.** `2_as_added.png`: the same as 1. The composition's motion blur is off, so with
   Override off there is nothing to blur.
3. **Override at 180.** `3_override_180.png`: the square smeared to the right (the way it moves)
   by about 14 pixels, fading out at the front edge.
4. **Override at 360.** `4_override_360.png`: a longer smear, about 30 pixels, smoother.
5. **In the app.** Precompose a layer that moves, add CC Force Motion Blur (Time) to the new
   composition layer, turn Override Native Shutter on and play: the movement inside is blurred.
   Put the effect on an ordinary layer instead: nothing changes and a warning names the effect.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: 8 samples and 180 degrees when added, and that the blur
trails ahead of the frame (the shutter opens on the frame, as CycoreFX ignores the phase).

## Speed

Measured while other lanes were building, so provisional: on the reference shot at 1920 by 1080,
with 8 samples the effect costs about 350 ms a frame when each frame is new, far slower than
real time; played a second time in order, the viewer keeps the blurred pictures and it costs
about 3 ms a frame (`verification/B-470_force_motion_blur_timing_table.md`). Drawing the
in-between pictures on the card would be a separate piece of work.
