# B-296: Grid

Built on 2026-10-10 as D-417, under your /loop request: After Effects' Grid, in **Generate**.
It draws a grid of lines, like graph paper, either in place of the layer or laid over it.

- **Anchor** (50, 50 per cent): a point the lines cross at; move it and the whole grid slides.
- **Size From** (Corner Point): how the cell size is set, exactly as in Checkerboard. Corner
  Point: the cell runs from the Anchor to the **Corner** (60, 60 per cent). Width Slider: square
  cells **Width** across (64). Width & Height Sliders: cells **Width** by **Height** (64 by 64).
- **Border** (2): how thick the lines are; 0 makes the grid disappear.
- **Feather Width**, **Feather Height** (0, 0): soften the upright lines and the level ones.
- **Invert Grid** (Off): On swaps them, so the cells are filled and the lines are empty.
- **Color** (white), **Opacity** (100).
- **Blending Mode** (None): None shows the grid alone in place of the layer; Normal, Add,
  Multiply, Screen, Overlay and Soft Light lay it over the layer; Stencil Alpha shows the layer
  only through the lines.

Adobe's page describes the effect in words and gives no formula, so how each pixel is worked out
is our own rule, written down in document 21. A draft draws everything at half size, the same
grid a little rougher.

The check, `verification/D-417_grid_table.md` (260 of 260), holds every pixel to numbers worked
out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-417 pictures/`
(`1_before.png` is the street with nothing on it). A see-through part shows as white or as your
viewer's checker.

## What to check

1. **As added.** `2_as_added.png`: thin white lines on a see-through layer, the street gone. On
   a white viewer background this looks blank white; on a checker you see the white lines.
2. **Over the street.** `3_normal_lines.png`: the street with square white graph-paper lines over
   it, slightly see-through.
3. **Soft, Multiply.** `4_feathered_multiply.png`: soft dark violet lines over the street, the
   cells twice as wide as tall.
4. **Inverted.** `5_inverted_dark.png`: the cells darkened, thin bright lines of the street left
   between them.
5. **Stencil.** `6_stencil.png`: the street seen only through thick lines, the cells see-through.
6. **In the app.** Add Grid (Generate) to a layer: white lines replace it. Set Blending Mode to
   Normal: the lines lie over the layer. Drag Anchor: the grid slides. Set Size From to Width
   Slider and drag Width: the cells grow and shrink. Raise Border, then the feathers: thicker,
   then softer lines. Turn Invert Grid on: the cells fill instead. Key Anchor across a few
   seconds and play: the grid moves. Switch to Draft: the same grid.
7. **Out of range.** Type 0 in Width: it is refused with a sentence saying it runs from 1 to
   10000.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

Measured only provisionally so far: other programs were building at the time
(`verification/B-296_grid_timing_table.md`). On the test shot (three 1920 by 1080 layers), a
Grid cost about 2 ms a layer a frame on the graphics card, well under what playback allows.
