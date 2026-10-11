# B-341: Hex Tile

Built on 2026-10-11 as D-461, under your /loop request and your go-ahead: **Hex Tile**, in
**Stylize**, after After Effects' CC HexTile. The picture is repeated in a honeycomb of
hexagons, each showing the part of the picture round the centre point.

- **Render** (Tile): Tile repeats the same hexagon everywhere; Fold Aligned fills each hexagon
  with six turned copies of one slice; Fold Seamlessly mirrors every other slice, so the slices
  and the hexagons meet with no seam.
- **Radius** (50, 1 to 4000 pixels): the size of the hexagons; a bigger one holds more of the picture.
- **Center** (50%, 50%): which part of the picture the hexagons show, and where the honeycomb starts.
- **Lock Center Tile** (off): on, the honeycomb starts at the layer's middle whatever Center says.
- **Rotate** (0 degrees): turns the honeycomb round its centre.
- **Smearing** (0%, 0 to 100): pulls each hexagon towards its own part of the picture, so the
  hexagons differ; at 100 with Render Tile, the picture comes back as it was.

The controls are After Effects' own; how it draws is our own rule beyond what the manual says,
and the starting values are ours (After Effects' are not published), so it is not matched to
After Effects pixel for pixel. At Draft a radius under 4 is too small to draw and Hex Tile is
left out with a warning, as Rain does with a small spacing.

The check, `verification/D-461_hex_tile_table.md` (153 of 153), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-461 pictures/`, over the
street (`town.png`; `1_before.png` without the effect).

## What to check

1. **Tiles.** `2_tile_radius_20.png` (Tile, radius 20): the middle of the street repeated in a
   honeycomb, every hexagon the same.
2. **Folds.** `3_fold_aligned.png` and `4_fold_seamless.png` (radius 20): each hexagon made of
   six slices; in the seamless one the slices mirror each other with no hard seam.
3. **Turned.** `5_rotate_30.png` (Rotate 30): the honeycomb tilted.
4. **Smeared.** `6_smearing_50.png` (Smearing 50): each hexagon shows a different part of the street.
5. **Locked.** `7_locked_center_30_40.png` (Center 30, 40, Lock Center Tile on): the hexagons show
   the street left of and above the middle, but the honeycomb is still centred on the picture.
6. **In the app.** Add Hex Tile (Stylize) to a picture, drag Radius, switch Render, key Rotate
   from 0 to 360 over two seconds and play it.
7. **Out of range.** Type 0 in Radius: it is refused with a sentence giving the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-341_hex_tile_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Hex Tile on three layers, played again, 57.6 ms a frame on the card at
radius 50 against 69.3 for the Noise alone; on the processor 156.0 against
97.0: a busy machine cannot say whether the card is quicker than the processor for this effect.
