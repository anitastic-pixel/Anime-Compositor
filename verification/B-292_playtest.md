# B-292: Checkerboard

Built on 2026-10-10 as D-413, under your /loop request: After Effects' Checkerboard, in
**Generate**. It draws a grid of coloured and clear rectangles, in place of the layer or laid over
it.

- **Anchor** (50, 50 per cent): where one coloured square starts. Moving it slides the grid.
- **Size From** (Width Slider): Width Slider makes squares; Width & Height Sliders rectangles;
  Corner Point makes each rectangle the size of the box between Anchor and **Corner**.
- **Width**, **Height** (64 pixels): the size of a rectangle.
- **Feather Width**, **Feather Height** (0): softens the edges, across and down.
- **Color** (white), **Opacity** (100).
- **Blending Mode** (None): None puts the grid in the layer's place, the clear squares see-through.
  Normal, Add, Multiply, Screen, Overlay and Soft Light lay it on the layer; Stencil Alpha shows the
  layer only through the squares.

After Effects also offers Hard Light, Color Dodge, Color Burn, Darken, Lighten, Difference,
Exclusion, Hue, Saturation, Color, Luminosity and Silhouette Alpha here; the program has none of
those blends yet, so they are not offered.

The check, `verification/D-413_checkerboard_table.md` (235 of 235), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-413 pictures/` (`town.png` is the
layer). In these pictures a see-through part shows as white or as your viewer's checker.

## What to check

1. **As added.** `2_as_added.png`: white squares 64 across, the rest see-through. Of the four
   squares meeting at the middle, the bottom-right one is white (the square that starts at the
   anchor).
2. **Normal, feathered.** `3_normal_feathered.png`: soft orange squares laid over the street at
   70 per cent, the street seen in the gaps, edges softer across than down.
3. **Corner Point.** `4_corner_point.png`: rectangles sized by the box between Anchor and Corner.
4. **Overlay.** `5_overlay.png`: the grid tints the street rather than covering it.
5. **Stencil Alpha.** `6_stencil.png`: the street seen only through soft squares.
6. **In the app.** Add Checkerboard (Generate) to a layer: white squares in its place. Drag Anchor:
   the grid slides. Set Blending Mode to Normal and Opacity to 50: the layer shows through. Key
   Width from 20 to 200 and play: the squares grow. Switch to Draft: the same grid, a little
   softer.
7. **Out of range.** Type 0 in Width: it is refused with a sentence saying it runs from 1 to
   10000.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-292_checkerboard_timing_table.md` (quiet machine): the reference shot with a moving Noise and a Checkerboard on three
layers, played again, 17.8 ms a frame on the card as added against 12.2 for the Noise alone, and
22.7 with Overlay; on the processor 53.5 and 78.6 against 44.9.
