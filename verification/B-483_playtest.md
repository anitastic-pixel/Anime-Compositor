# B-483: Warp

Built on 2026-10-12 as D-604, the third effect of mesh and brush warping (P0-18). Warp is in the
Distort group. It bends the layer into one of fifteen ready-made shapes, with the names After
Effects uses (Arc, Arc Lower, Arc Upper, Arch, Bulge, Shell Lower, Shell Upper, Flag, Wave, Fish,
Rise, Fisheye, Inflate, Squeeze, Twist), with Bend, Warp Axis and two Distortions. Adobe does not
publish how each shape is worked out, so the shapes are this program's own and may differ a little
from After Effects'.

The check, `verification/D-604_warp_table.md` (176 of 176), holds every pixel to numbers a separate
program worked out before the code was written, and holds the graphics card's picture to the
processor's (51 of 51 within 1 level). The pictures are in `verification/D-604 pictures/`.

## What to check

1. **`1_before.png`**: the town street, no effect.
2. **`2_arc.png`**: Warp as added (Arc, Bend 50): the street bent round a circle, the bottom
   corners lifted clear (see-through).
3. **`3_flag.png`**: Flag at Bend 50: the street rising then dropping in one wave across.
4. **`4_flag_vertical.png`**: the same Flag on the vertical axis: the wave runs down instead.
5. **`5_twist.png`**: Twist at Bend 100: the middle turned round, the corners nearly in place.
6. **`6_horizontal_distortion_60.png`**: Bend 0, Horizontal Distortion 60: the right side taller
   than the left.
7. **In the app.** Add Warp to a layer: it bends into an arc at once. Pick other styles from the
   Warp Style list; Bend 0 puts the layer back.

## Speed

`verification/B-483_warp_timing_table.md`: PROVISIONAL, the machine busy with other lanes (6.6 GB of the card's memory in use by other programs): on the card 27.0 ms a frame as added (Arc, Bend 50) and 56.5 for Twist 80 with both distortions at 30, against 18.0 for the Noise alone, about 3 ms a 1080p layer for the Arc; the processor 192.1 and 189.5 against 100.8, about 30 ms a layer.
