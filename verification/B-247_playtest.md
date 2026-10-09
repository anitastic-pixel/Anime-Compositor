# B-247: Kernel

Built on 2026-10-09 under your effects loop request, decided as D-368. **Kernel** (in **Color
Correction**) is our version of CycoreFX's CC Kernel: each pixel is rebuilt from itself and its
eight neighbours, each weighed by a number in a three by three grid you type in, then divided.
With it you make your own blurs, sharpens, edge finders and embosses.

The settings, with the values they start at:

- **Line 1, Line 2, Line 3** (0 0 0, 0 1 0, 0 0 0): the grid's rows, top to bottom, each three
  numbers from -1000 to 1000 for the left, middle and right pixel. Line 2's middle number is the
  pixel itself. As it starts the pixel keeps only itself, so nothing changes.
- **Divider** (1): 0.01 to 1000; the weighed sum is divided by it. Make it the sum of the grid's
  numbers to keep the brightness the same.
- **Absolute Values** (Off): On makes a negative result positive, for edge grids.

The check, `verification/D-368_kernel_table.md` (135 of 135), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-368 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: exactly the same as before.
2. **Blur.** `3_box_blur.png`, all nine numbers 1 and Divider 9: every edge softened by a pixel.
3. **Sharpen.** `4_sharpen.png`, 0 -1 0, -1 5 -1, 0 -1 0: every edge crisper.
4. **Edges.** `5_edges.png`, -1 all round and 8 in the middle, Absolute Values On: the flat road
   and walls go black; only the outlines of houses, windows and markings are left.
5. **Emboss.** `6_emboss.png`, -2 -1 0, -1 1 1, 0 1 2: edges lit from the lower right,
   shadowed at the upper left.
6. **In the app.** Type the sharpen grid, then key Line 2's middle number from 1 to 9 over two
   seconds: the street goes from nearly black at 1, through the sharpened street at 5, to
   washed out at 9.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Kernel costs about 4.3 to 5.5 ms a 1080p layer, a little above our 4 ms
target for neighbourhood effects, and about 10.5 ms on the processor
(`verification/B-247_colour_timing_table.md`).

## Not built

- The grid weighs the picture's sRGB values, where CycoreFX weighs its stored values; results
  are held between black and white, so a negative sum is black.
- CycoreFX's Blend w. Original is the Mix every effect has.
- No Offset or Alpha setting: the CycoreFX manual (HD 1.8.9) has neither for CC Kernel.
- No ready-made grids to pick from; type the numbers.
