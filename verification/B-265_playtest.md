# B-265: Griddler

Built on 2026-10-09 under your effects loop request, decided as D-386. **Griddler** (in
**Distort**) is our version of CycoreFX's CC Griddler: the picture cut into a grid of square
tiles, each tile's piece scaled and turned about the tile's own centre, like a wall of small
screens. The formulas are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **Horizontal Scale** (80) and **Vertical Scale** (80): -1000 to 1000 per cent, how big each
  tile's piece is drawn across and down. Below 100 leaves gaps between the tiles; a negative
  number turns each piece over.
- **Tile Size** (10): 0.1 to 100 per cent of the layer's width, how big each square tile is.
  The tiles are measured from the layer's top-left corner.
- **Rotation** (0): degrees, turning each tile's piece about its own centre.
- **Cut Tiles** (On): On keeps each piece inside its own tile, with clear gaps where it does not
  reach; Off lets each tile show more of the picture round its centre instead.

The layer does not grow.

The check, `verification/D-386_griddler_table.md` (158 of 158), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-386 pictures/`. They are
saved over white, so parts left see-through show as white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the street as a grid of ten tiles across, each piece drawn
   at 80 per cent with white seams between them.
2. **Scales 100.** `3_scale_100.png`: exactly the same as before.
3. **Turned, uncut.** `4_turned_45_uncut.png`, Tile Size 20, scales 100, Rotation 45, Cut
   Tiles Off: five big tiles across, each holding its piece of the street turned on the slant,
   no gaps inside the tiles.
4. **Mirrored.** `5_mirrored_across.png`, Tile Size 25, Horizontal Scale -100: each tile's piece
   of the street flipped left to right.
5. **In the app.** Add Griddler to a layer and key Rotation from 0 to 360 over two seconds:
   every tile spins in place. Key the scales from 100 down to 0: the tiles shrink away to
   nothing.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Griddler costs about 2.8 ms a 1080p layer, and about 17 ms on the processor
(`verification/B-264_distort_timing_table.md`; provisional, the machine was busy).

## Not built

- The rule is our own reading of CycoreFX's one-sentence descriptions; no tutorial with numbers
  was matched against it.
- With Cut Tiles off each tile shows the picture round its own centre, not repeated copies of
  its piece.
- The tile edges are hard, not smoothed.
