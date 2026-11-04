# B-950 playtest: Motion Tile, Phase and Horizontal Phase Shift (D-1070)

The tutorial (lHQvi94dpvM, about 4:00) shows Motion Tile's Phase and Horizontal Phase Shift in the
panel. They slide every other column (or row) of tiles to make a brick pattern.

## Steps

1. Make a solid with a Linear Wipe at 50 and angle 45, so each tile is a half-dark triangle. Add
   Motion Tile after it and set Tile Width and Tile Height to 25. You see a regular grid of
   triangles (the left shot in the picture).
2. Find the two new settings in Motion Tile's panel. **Phase** shows a dial at 0, and
   **Horizontal Phase Shift** shows Off.
3. Set Phase to 180. Every other column of tiles slides down by half a tile.
4. Set Horizontal Phase Shift to On. Now every other row slides right by half a tile instead,
   like bricks in a wall (the middle shot).
5. Set it back to Off and Phase to 90. Every other column slides down by a quarter tile (the
   right shot).
6. Set Phase to 360. The picture is the same as at 0.
7. Key Phase from 0 to 360 over two seconds and play. Every other column scrolls down smoothly
   while the columns between them stand still.
8. Turn Mirror on with Phase 180. The slid columns still turn over tile by tile.
9. Save, close and open the project. Phase and Horizontal Phase Shift come back as you left them.

## What you should see

`verification/B-950 pictures/bricks.png` holds three shots, left to right. The first is
plain tiles at 25. The second is Phase 180 with Horizontal Phase Shift on: every other row is half
a tile to the right. The third is Phase 90: every other column is a quarter tile down.

Wrong if:
- step 3 moves every column, or none;
- step 4 still slides columns;
- step 6 differs from step 1;
- a project saved before this change looks different.

Not the same as After Effects:
- The direction of the slide (down, or right) is our choice. After Effects publishes no formula.
- With the graphics card on, a nonzero Phase is drawn on the processor, so it may be a little
  slower.
