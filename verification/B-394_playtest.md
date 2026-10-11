# B-394: Stripe Sweep

Built on 2026-10-11 as D-514, under your /loop request: Stripe Sweep, in **Transition**, our own
take on CycoreFX's CC Line Sweep (a name of our own). The layer is cut into lines and each line
is wiped away from one end to the other, the lines starting one after another.

- **Completion** (0): how far the wipe has gone; 0 the layer whole, 100 all gone.
- **Direction** (0): the way the lines sweep; 0 is left to right, 90 top to bottom.
- **Thickness** (20): how thick each line is, in pixels.
- **Slant** (50): how much the lines overlap. At 0 each line waits for the one before to
  finish; at 99 they all go together, one straight edge.
- **Flip Direction** (Off): on, the lines sweep the other way.

Each line's edge is hard. The layer does not grow.

CycoreFX publishes no formula and no settings when added, so the rule and those settings are
ours, written down in document 21. CycoreFX says Direction and Thickness can not be animated;
here they can.

The check, `verification/D-514_stripe_sweep_table.md` (157 of 157), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-514 pictures/`; the checkerboard or white you see in a viewer is where the layer
is gone.

## What to check

1. **The street.** `0_the_street.png`: the shot without the effect.
2. **Stripes.** `1_stripes.png`: completion 40 as added: the top lines gone, each line below
   less far along than the one above, a staircase edge.
3. **One at a time.** `2_one_at_a_time.png`: slant 0: the top lines all gone, one line part
   way, the lines below not started.
4. **All together.** `3_all_together.png`: slant 99: every line the same, a straight edge.
5. **Columns, flipped.** `4_columns_flipped.png`: direction 90, thickness 40, flipped: columns
   wiped upward from the bottom, the right-hand ones first.
6. **Turned.** `5_turned.png`: direction 30, completion 50: the lines slanting.
7. **Done.** `6_done.png`: completion 100, nothing left.
8. **In the app.** Add Stripe Sweep (Transition) to a layer over another, keyframe Completion
   from 0 to 100 and play. Try a few Slants, a thin and a thick line, a Direction and Flip.
9. **Out of range.** Type 100 in Slant: it is refused with a sentence saying the range.
10. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; that the lines are laid out from
the layer's middle.

## Speed

`verification/B-394_stripe_sweep_timing_table.md`. Three Stripe Sweeps on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL (machine not quiet, other lanes' tests and builds running), card 101.0, 116.8 and 141.2 ms a frame (as added at completion 50; thickness 4, slant 0, direction 30; flipped, thickness 200, slant 99) against 76.2 for the Noise alone, processor 152.1, 147.1 and 146.3 against 143.5; a quiet re-measure is owed.
