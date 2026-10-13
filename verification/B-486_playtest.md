# B-486: Mask Smear

Built on 2026-10-13 as D-607, the sixth and last effect of mesh and brush warping (P0-18). It is
After Effects' Smear; in this program it is called Mask Smear, because our Smear (D-392, after CC
Smear) already has the name. It is in the Distort group. The picture inside one of the layer's
masks (the source) is carried along as that mask is moved, turned and scaled; the rest of the
picture follows, and a second mask (the boundary) holds everything outside it still. It has After
Effects' Source Mask, Boundary Mask, Mask Offset, Mask Rotation, Mask Scale, Percent and
Elasticity. Adobe does not publish how it is worked out, so the pull is this program's own.

The check, `verification/D-607_mask_smear_table.md` (144 of 144), holds every pixel to numbers a
separate program worked out before the code was written, and holds the graphics card's picture to
the processor's (36 of 36 within 1 level). The pictures are in `verification/D-607 pictures/`.

## What to check

1. **`1_before.png`**: the town street, no effect. Mask Smear as added draws exactly this.
2. **`2_carried_right.png`**: a square about the middle moved 60 pixels right: the middle of the
   street carried right, the edges of the picture held.
3. **`3_within_boundary.png`**: the same, held at a boundary mask from 120 to 420 across: nothing
   left or right of it moves.
4. **`4_turned.png`**: the square turned 45 degrees clockwise: the middle of the street twisted.
5. **`5_swollen.png`**: the square scaled to 160 per cent: the middle swollen.
6. **`6_stiff.png`** and **`7_liquid.png`**: picture 4 at Elasticity Stiff (the twist spread wide)
   and Liquid (kept near the square's edges): the two look different.
7. **In the app.** Draw a mask on a layer, add Mask Smear, set Source Mask 1, then change Mask
   Offset, Mask Rotation or Mask Scale: the picture inside the mask moves with it. Percent 0 puts
   it back.

Not settled, so this is marked partial: the mask turns and scales about the middle of its outline,
which may not be exactly where After Effects turns it, and the masks are chosen by number.

## Speed

`verification/B-486_mask_smear_timing_table.md`: PROVISIONAL, the machine busy with other lanes (5.3 GB of the card's memory in use by other programs): on the card 52.9 ms a frame for the square carried 200 right and 36.7 for the same turned, scaled and within a boundary at Liquid, 60 per cent, against 15.4 for the Noise alone; the processor 91.1 and 85.1 against 50.2.
