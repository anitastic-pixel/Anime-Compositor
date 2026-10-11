# B-395: Push Iris

Built on 2026-10-11 as D-515, under your /loop request: Push Iris, in **Transition**, our own
take on CycoreFX's CC Radial ScaleWipe (a name of our own). A round hole opens at the Center and
pushes the picture out of its way, the middle of the picture stretched round the hole's edge.
Reversed, the outside closes in like an iris and pulls the picture toward the middle.

- **Completion** (0): how far the wipe has gone; 0 the layer whole, 100 all gone.
- **Center** (the middle): where the hole opens.
- **Reverse Transition** (Off): on, an iris closing in instead of a hole opening.

The hole's edge is hard. The layer does not grow.

CycoreFX says only "radial wipe where edge folds" and publishes no formula and no settings when
added, so the rule and those settings are ours, written down in document 21.

The check, `verification/D-515_push_iris_table.md` (118 of 118), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-515 pictures/`; the checkerboard or white you see in a viewer is where the layer
is gone.

## What to check

1. **The street.** `0_the_street.png`: the shot without the effect.
2. **A hole.** `1_hole.png`: completion 30: a round hole in the middle, the street pushed out
   round it, its middle stretched at the hole's edge.
3. **A bigger hole.** `2_bigger_hole.png`: completion 60: the hole wider, more of the street
   pushed off the edges.
4. **Iris closing.** `3_iris_closing.png`: completion 30 reversed: the corners gone, the street
   squeezed toward the middle.
5. **Off centre.** `4_off_centre.png`: Center at 20, 70: the hole opens low on the left.
6. **Done.** `5_done.png`: completion 100, nothing left.
7. **In the app.** Add Push Iris (Transition) to a layer over another, keyframe Completion from
   0 to 100 and play. Move the Center; try Reverse Transition.
8. **Out of range.** Type 150 in Completion: it is refused with a sentence saying the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; that the picture is pushed
outward, not only cut.

## Speed

`verification/B-395_push_iris_timing_table.md`. Three Push Irises on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL (machine not quiet, other lanes' tests and builds running), card 102.7, 78.7 and 120.1 ms a frame (as added at completion 50; reversed; completion 30, Center [20, 70]) against 129.1 for the Noise alone, processor 218.4, 233.6 and 179.5 against 140.8; a quiet re-measure is owed.
