# B-484: Liquify

Built on 2026-10-12 as D-605, the fourth effect of mesh and brush warping (P0-18). Liquify is in
the Distort group. A brush stroke over the layer pushes, turns, pinches or swells the picture
under it. It has six of After Effects' brushes (Warp, Twirl Clockwise, Twirl Counterclockwise,
Pucker, Bloat, Shift Pixels) with Brush Size, Brush Pressure, Distortion Percentage and Distortion
Mesh Offset. The stroke is kept as a list of numbers. Adobe does not publish how it is worked out,
so the brushes are this program's own.

The check, `verification/D-605_liquify_table.md` (147 of 147), holds every pixel to numbers a
separate program worked out before the code was written, and holds the graphics card's picture to
the processor's (42 of 42 within 1 level). The pictures are in `verification/D-605 pictures/`.

## What to check

1. **`1_before.png`**: the town street, no effect. Liquify as added draws exactly this.
2. **`2_warp.png`**: the Warp brush stroked across the middle: the picture dragged along the
   stroke.
3. **`3_twirl_clockwise.png`**: Twirl Clockwise at one spot: the picture there turned clockwise.
4. **`4_twirl_counterclockwise.png`**: the same spot turned the other way.
5. **`5_pucker.png`**: Pucker at one spot: the picture drawn in toward it, shrunk.
6. **`6_bloat.png`**: Bloat at one spot: the picture swelled out from it, magnified.
7. **`7_shift_pixels.png`**: Shift Pixels stroked across: the picture moved to the side of the
   stroke.
8. **In the app.** Add Liquify: nothing changes until a stroke is typed. Its Stroke is a list of
   numbers, x then y for each point, in per cent of the layer.

Not settled, so this is marked partial: After Effects has more brushes (Turbulence, Reflection,
Clone, Reconstruction), freeze and thaw areas, many strokes, and lets you paint on the viewer;
this has six brushes, one stroke, and the stroke is typed.

## Speed

`verification/B-484_liquify_timing_table.md`: PROVISIONAL, the machine busy with other lanes (13.1 GB of the card's memory in use by other programs): on the card 158.0 ms a frame for a Warp stroke across the middle (brush 200) and 73.1 for one Twirl dab (brush 600), against 66.1 for the Noise alone; the processor 248.2 and 158.2 against 99.4.
