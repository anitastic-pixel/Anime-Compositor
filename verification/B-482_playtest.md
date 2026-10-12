# B-482: Mesh Warp

Built on 2026-10-12 as D-603, the second effect of mesh and brush warping (P0-18). Mesh Warp is in
the Distort group. It lays a grid over the layer (7 by 7 when added); move the grid's points and
the picture bends smoothly with them. It has After Effects' Rows, Columns and Quality; the grid's
points are kept as a list of numbers. Adobe does not publish how it is worked out, so the bending
is this program's own.

The check, `verification/D-603_mesh_warp_table.md` (130 of 130), holds every pixel to numbers a
separate program worked out before the code was written, and holds the graphics card's picture to
the processor's (39 of 39 within 1 level). The pictures are in `verification/D-603 pictures/`.

## What to check

1. **`1_before.png`**: the town street, no effect. Mesh Warp as added draws exactly this.
2. **`2_middle_pushed.png`**: a 2 by 2 grid with its middle point pushed right: the street bowed
   right round the middle, its outline kept.
3. **`3_edges_waving.png`**: a 3 by 3 grid with the top edge's points moved down and up: the top
   edge waves, clear (see-through) above it where it dips.
4. **`4_folded.png`**: the middle point dragged far past the edge: the picture folds over itself.
5. **`5_middle_pushed_quality_1.png`**: picture 2 at Quality 1: the bow drawn in straight pieces.
6. **In the app.** Add Mesh Warp: nothing changes. Its Distortion Mesh is a list of numbers, x then
   y for each point, row by row, in per cent of the layer.

Not settled, so this is marked partial: After Effects gives each grid point its own curve handles
and lets you drag the points on the viewer; this has neither yet. Changing Rows or Columns once a
mesh is written is refused, with a sentence saying how many numbers the new grid needs, until the
list is cleared or rewritten.

## Speed

`verification/B-482_mesh_warp_timing_table.md`: PROVISIONAL, the machine busy with other lanes: on the card 46.1 ms a frame with a 2 by 2 mesh and 37.8 with 7 by 7, against 50.0 for the Noise alone, the machine's own swing larger than the effect; the processor 159.4 and 160.8 against 99.3, about 60 ms added over three 1080p layers, about 20 a layer.
