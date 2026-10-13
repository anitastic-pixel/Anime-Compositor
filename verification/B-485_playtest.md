# B-485: Reshape

Built on 2026-10-13 as D-606, the fifth effect of mesh and brush warping (P0-18). Reshape is in
the Distort group. It pulls the picture inside one of the layer's masks (the source) into the
shape of another (the destination); the rest of the picture follows, and a third mask (the
boundary) holds everything outside it still. It has After Effects' Source Mask, Destination Mask,
Boundary Mask, Percent and Elasticity. Adobe does not publish how it is worked out, so the pull is
this program's own.

The check, `verification/D-606_reshape_table.md` (142 of 142), holds every pixel to numbers a
separate program worked out before the code was written, and holds the graphics card's picture to
the processor's (36 of 36 within 1 level). The pictures are in `verification/D-606 pictures/`.

## What to check

1. **`1_before.png`**: the town street, no effect. Reshape as added draws exactly this.
2. **`2_pulled_right.png`**: a square about the middle pulled to the same square 60 pixels right:
   the middle of the street dragged right, the edges of the picture held.
3. **`3_within_boundary.png`**: the same, held at a boundary mask from 120 to 420 across: nothing
   left or right of it moves.
4. **`4_squeezed_tall.png`**: the square pulled into a tall strip: the middle squeezed across and
   stretched up and down.
5. **`5_stiff.png`** and **`6_liquid.png`**: picture 2 at Elasticity Stiff (the pull spread wide)
   and Liquid (kept near the square's edges): the two look different.
6. **In the app.** Draw two masks on a layer, add Reshape, set Source Mask 1 and Destination Mask 2:
   the picture inside the first mask moves into the second's shape. Percent 0 puts it back.

Not settled, so this is marked partial: After Effects lets you place matching points between the
two outlines (Correspondence Points) and choose how they are matched; here the outlines are matched
by length from their first points, and the masks are chosen by number.

## Speed

`verification/B-485_reshape_timing_table.md`: PROVISIONAL, the machine busy with other lanes (11.0 GB of the card's memory in use by other programs): on the card 92.2 ms a frame for the square pulled right and 153.5 for the same within a boundary at Liquid, 60 per cent, against 45.1 for the Noise alone; the processor 424.3 and 382.4 against 205.8.
