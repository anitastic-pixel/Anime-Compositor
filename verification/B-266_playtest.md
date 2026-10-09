# B-266: Fisheye

Built on 2026-10-09 under your effects loop request, decided as D-387. **Fisheye** (in
**Distort**) is our version of CycoreFX's CC Lens: a round lens over the picture that swells
the middle out like a fish-eye or a glass ball, or pinches it in like a dish, with everything
outside the circle left clear. The formulas are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **Center** (50, 50): per cent of the layer's width and height, where the lens sits.
- **Size** (50): 0 to 1000, the lens's radius as a share of half the layer's diagonal. 100
  reaches the corners; 0 shows nothing.
- **Convergence** (50): -100 to 100. Above 0 the middle swells out like a fish-eye, 100 like
  a glass ball; below 0 it is pinched in like a dish; 0 leaves the picture inside the lens
  as it is.

The lens's rim is softened over one pixel. The layer does not grow.

The check, `verification/D-387_fisheye_table.md` (149 of 149), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-387 pictures/`. They are
saved over white, so parts left see-through show as white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the middle of the street swells out inside a circle, the
   corners outside it white.
2. **A glass ball.** `3_ball_full.png`, Size 120, Convergence 100: the street seen through a
   glass ball: the middle big, the road curving down at the sides, every corner filled.
3. **A dish.** `4_dish.png`, Size 100, Convergence -100: the street pinched small in the middle,
   the buildings shrunk; near the edges the lens reads past the street, so curved white edges
   show.
4. **In the app.** Add Fisheye to a layer and key Convergence from -100 to 100 over two
   seconds: the picture goes from pinched to swollen. Drag Center about: the lens follows.
5. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Fisheye costs about 1.7 ms a 1080p layer, and 3 to 12 ms on the processor
(`verification/B-264_distort_timing_table.md`; provisional, the machine was busy).

## Not built

- The lens curve is our own reading of CycoreFX's one-sentence descriptions; no tutorial with
  numbers was matched against it.
- Outside the lens the layer is always clear; there is no setting to keep the picture there.
