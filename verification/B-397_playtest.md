# B-397: Twister

Built on 2026-10-11 as D-517, under your /loop request: Twister, in **Transition**, our own
take on CycoreFX's CC Twister (a name of our own). The layer twists over round a line through
the Center, a strip at a time like a ribbon turning, and shows another layer on its back.

- **Completion** (0): how far the twist has gone; at 100 the layer has turned over entirely.
- **Backside** (none): the layer seen on the back. None leaves the back empty; the layer itself
  twists into itself; a layer of another size is shown centred.
- **Shading** (On): the strip darkens as it turns edge on, and comes back on the far face.
- **Center** (the middle): where the line is.
- **Axis** (0): the way the twist runs; 0 is from left to right, 90 from top to bottom.

The layer does not grow.

CycoreFX publishes no formula and no settings when added, so the rule and those settings are
ours, written down in document 21.

The check, `verification/D-517_twister_table.md` (203 of 203), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-517 pictures/`.

## What to check

1. **The street and its back.** `1_street.png` is the shot; `2_reveal.png`, the same street in
   its opposite colours, is the layer on the back.
2. **Begun.** `3_begun.png`: completion 25: the left half turning edge on, squeezed toward the
   middle row and darker; the right half untouched.
3. **Half over.** `4_half_over.png`: completion 50: the left half turned over to the opposite
   colours, the right half starting to turn.
4. **Down, unshaded.** `5_down_unshaded.png`: completion 50, axis 90, shading off: the twist runs
   downward, the top half turned over, no darkening.
5. **Done.** `6_done.png`: completion 100: the opposite colours everywhere.
6. **In the app.** Add Twister (Transition) to a layer over another, choose a Backside, keyframe
   Completion from 0 to 100, and play. Try an Axis and the Center.
7. **Out of range.** Type 101 in Completion: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; a flat twist, squeezed rather
than drawn in perspective; the back shown as it is, not mirrored.

## Speed

`verification/B-397_twister_timing_table.md`. Three Twisters on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL (machine not quiet, another lane's cargo run sharing the machine), card 168.6, 57.6 and 200.2 ms a frame (completion 50, layer 4 on the back; completion 50, axis 30, shading off, nothing on the back; completion 75, Center [30, 60], layer 4 on the back) against 64.5 for the Noise alone, processor 204.4, 139.1 and 184.9 against 113.3; a quiet re-measure is owed.
