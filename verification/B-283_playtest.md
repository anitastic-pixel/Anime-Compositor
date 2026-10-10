# B-283: Tiles

Built on 2026-10-09, decided as D-404 under your /loop request. **Tiles** (in **Distort**) is our
version of CycoreFX's CC Tiler: the layer shrunk and repeated in a grid of copies across its own
size. It uses Motion Tile's tiling rule, so with no blend it gives exactly Motion Tile's picture
for the same tile size.

The settings, with the values they start at:

- **Scale** (50): per cent, how big each copy is. 50 gives copies at half size, 2 by 2; 25 gives
  4 by 4. 100 is the layer as it is. The least is 1.
- **Center** (50, 50): per cent of the layer's width and height, where one copy is centred.
  Moving it slides the whole grid; copies leaving one side come back on the other.
- **Blend w. Original** (0): per cent, how much of the untiled layer shows through. 100 is the
  layer as it is.

The check, `verification/D-404_tiles_table.md` (150 of 150), holds every pixel to numbers worked
out by a separate program before the code existed, the graphics card's picture to within 1 level
of the processor's, and compares Tiles with Motion Tile on three street pictures (identical byte
for byte). The pictures are in `verification/D-404 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`, scale 50: the street at half size, repeated; you see two
   streets down and copies across (the houses repeat along the street, so the joins across are
   hard to spot).
2. **Smaller.** `3_scale_25.png`: four streets down, each a quarter of the height.
3. **Off centre.** `4_off_centre.png`, scale 33, centre 20, 30: about three streets down, the
   grid slid so the copies at the top and bottom are cut off.
4. **Blend.** `5_blend_50.png`, scale 25, blend 50: the four small streets seen through the
   full-size street, both at half strength.
5. **Very small.** `6_scale_3.png`: copies so small the street becomes a fine striped texture.
6. **In the app.** Add Tiles to a layer, key Scale from 100 to 20 over a second and play: the
   layer shrinks into more and more copies. Then key Center across: the grid slides and wraps.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Tiles at scale 25 costs about 16 ms a 1080p layer on the graphics card and
about 28 ms on the processor; at scale 7 about 173 ms on the card
(`verification/B-283_tiles_timing_table.md`, provisional: the machine was busy). Very small
scales are slow because each pixel averages many points.

## Not built

- Scale below 1 per cent. CC Tiler goes down to 0, where tutorials show a fine dot look.
- The starting scale of 50 is ours; no source read gives CC Tiler's.
- No handle on the viewer for Center yet; set it in Effect Controls.
- The CC Tiler tutorials found are videos and were not watched; the pages read give no numbers
  to reproduce.
