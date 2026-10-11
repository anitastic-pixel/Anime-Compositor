# B-393: Glow Wipe

Built on 2026-10-11 as D-513, under your /loop request: Glow Wipe, in **Transition**, our own
take on CycoreFX's CC Light Wipe (a name of our own). The layer is wiped away from a centre point
outward, and the moving edge carries a band of light.

- **Completion** (0): how far the wipe has gone; 0 the layer whole, 100 all gone.
- **Center** (50, 50): where the wipe starts, in per cent of the layer's width and height.
- **Intensity** (100): how bright the light is; 0 is no light at all.
- **Direction** (0): turns the walls or the square (the circle has no direction).
- **Shape** (Radial): Doors (two walls of light moving apart), Radial (a circle) or Rectangular
  (a square).
- **Color from Source** (Off): on, the light is the layer's own colour where it is wiped.
- **Color** (white): the light's colour when Color from Source is off.
- **Reverse Transition** (Off): on, the wipe comes in from the edges and the middle stays last.

The band of light is a tenth of the wipe's range wide and fades out on both sides of the edge.
The edge itself is hard. The layer does not grow.

CycoreFX publishes no formula and no settings when added (beyond Color from Source off), so the
rule and those settings are ours, written down in document 21.

The check, `verification/D-513_glow_wipe_table.md` (177 of 177), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-513 pictures/`; the checkerboard or white you see in a viewer is where the layer is
gone.

## What to check

1. **The street.** `0_the_street.png`: the shot without the effect.
2. **A circle.** `1_circle.png`: a hole in the middle with a white glow round its edge, fading
   both into the hole and onto the street.
3. **Doors.** `2_doors.png`: two walls of light moving left and right from the middle.
4. **A turned square.** `3_square_turned.png`: a square of light turned 30 degrees.
5. **Orange light.** `4_orange_light.png`: the circle with an orange light at 300 intensity, a
   strong orange ring.
6. **Reversed, in its own colour.** `5_reversed_own_colour.png`: the outside wiped away first,
   the middle kept, the light the street's own colours.
7. **Done.** `6_done.png`: completion 100, nothing left and no light.
8. **In the app.** Add Glow Wipe (Transition) to a layer over another, keyframe Completion from 0
   to 100 and play. Try each Shape, Color from Source, a colour and Reverse Transition.
9. **Out of range.** Type 2000 in Intensity: it is refused with a sentence saying the range.
10. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; the width and fall-off of the
band; that the light is added on top.

## Speed

`verification/B-393_glow_wipe_timing_table.md`. Three Glow Wipes on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL (machine not quiet, other lanes' tests and builds running), card 139.5, 94.7 and 50.5 ms a frame (as added at completion 50; doors, direction 30, Color from Source; rectangular, reversed, orange at 300) against 97.5 for the Noise alone, processor 156.3, 202.0 and 295.8 against 125.7; a quiet re-measure is owed.
