# B-255: Color Stabilizer

Built on 2026-10-09 under your effects loop request, decided as D-376. **Color Stabilizer** (in
**Color Correction**) is our version of After Effects' Color Stabilizer: the colours at one, two
or three points of a reference frame are kept steady through a flickering shot.

The settings, with the values they start at:

- **Stabilize** (Brightness): Brightness moves every colour by how much the Black Point's sample
  got brighter or darker; Levels maps each colour so the Black and White Points match the
  reference; Curves maps it through all three points.
- **Reference Frame** (0): the composition frame whose colours are kept. After Effects has a Set
  Frame button; here you type the frame.
- **Black Point**, **Mid Point** and **White Point** ((25, 50), (50, 50), (75, 50)): where the
  samples are taken, in per cent of the layer's width and height. Put the black point on
  something dark and the white point on something light.
- **Sample Size** (5): the radius in pixels of each sample.

The check, `verification/D-376_color_stabilizer_table.md` (111 of 111), holds every pixel to
numbers worked out by a separate program before the code existed, and the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-376 pictures/`,
each pixel grown 24 times so you can see it: a small grey ramp whose frames flicker.

## What to check

1. **The reference.** `1_reference_frame_0.png`: frame 0, a warm grey ramp.
2. **A colour cast.** `2_frame_2_before.png`: frame 2, greenish and flatter. `3_frame_2_levels.png`,
   Levels to frame 0: the cast and flatness are taken out and it looks like frame 0 again.
3. **A brightness jump.** `4_frame_1_before.png`: frame 1, lifted brighter.
   `5_frame_1_brightness.png`, Brightness to frame 0: brought back down to frame 0's brightness.
4. **In the app.** Put an Exposure Flicker on a drawing, then a Color Stabilizer under it with
   Stabilize Levels and the points on a dark and a light part: play it; the flicker is mostly
   gone.
5. **On an adjustment layer.** Put it on an adjustment layer: nothing changes and a warning says
   it needs to be on the layer itself.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

**Timing.** On the reference shot, three 1080p layers with a moving Noise, played again at Full: 41.3 ms a frame without and 52.5 with Color Stabilizer when the processor draws everything, about 3.7 ms a layer; with the graphics card on, 58.5, a little slower than the processor alone, logged (`verification/B-253_colour_timing_table.md`).

## Not built

- Adobe does not publish its method, so the rule is ours (document 21). Curves joins the three
  samples with straight lines.
- A number for the reference frame, not a Set Frame button.
- On an adjustment layer it does nothing, with a warning (the frames beneath at other times are
  not to hand there).
- Drawn by the processor; the graphics card draws the rest of the frame round it, and such a
  frame is a little slower with the card on than with the processor alone (see Timing).
