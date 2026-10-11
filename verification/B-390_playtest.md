# B-390: Tile Wipe

Built on 2026-10-11 as D-510, under your /loop request: Tile Wipe, in **Transition**, our own
take on CycoreFX's CC Grid Wipe (a name of our own). The layer is cut into square tiles that
shrink away, the tiles nearest the centre point going first.

- **Completion** (0): how far the wipe has gone; 0 the layer whole, 100 all gone.
- **Center** (50, 50): where the wipe starts, in per cent of the layer's width and height.
- **Rotation** (0): turns the whole grid, not each tile.
- **Border** (0): how soft each shrinking tile's edge is, in pixels; 0 is a hard edge.
- **Tiles** (10): how many tiles across the layer's width.
- **Shape** (Radial): the order the tiles go in: Radial in a circle, Rectangle in a square,
  Doors from the centre line out to the left and right, like two doors opening.
- **Reverse Transition** (Off): On makes the tiles at the edges go first.

Not built: a control for how long each tile takes to shrink (always a quarter of the whole
wipe). The layer does not grow.

CycoreFX publishes no formula and no settings when added, so the rule and those settings are
ours, written down in document 21. Two readings are ours too: Tiles counts the tiles across the
width, and Border is the width of the soft edge.

The check, `verification/D-510_tile_wipe_table.md` (177 of 177), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-510 pictures/`; the checkerboard or white you see in a viewer is where the layer is
gone.

## What to check

1. **The street.** `0_the_street.png`: the shot without the effect.
2. **Radial, half way.** `1_radial_half.png`: a round hole of shrinking squares in the middle,
   the squares smaller nearer the middle.
3. **Rectangle, reversed.** `2_rectangle_reversed.png`: the edges going first, a square of the
   street left in the middle.
4. **Doors, soft border.** `3_doors_soft_border.png`: a band down the middle gone, the street
   left on both sides, the squares' edges soft.
5. **Many tiles, turned.** `4_many_tiles_turned.png`: forty small squares across, the whole grid
   turned 30 degrees.
6. **Done.** `5_done.png`: completion 100, nothing left.
7. **In the app.** Add Tile Wipe (Transition) to a layer over another, keyframe Completion from 0
   to 100 and play: the tiles shrink away from the middle out, showing the layer below. Try each
   Shape and Reverse Transition.
8. **Out of range.** Type 201 in Tiles: it is refused with a sentence saying the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; how long each tile takes to go;
how Border looks.

## Speed

`verification/B-390_tile_wipe_timing_table.md`. Three Tile Wipes on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL (machine not quiet, other lanes compiling): card 24.6, 22.6 and 28.1 ms a frame against 17.9 for the Noise alone, processor 63.8, 71.9 and 119.8 against 57.4; a quiet re-measure is owed.
