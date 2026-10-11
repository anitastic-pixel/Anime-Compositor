# B-350: Contour Paint

Built on 2026-10-11 as D-470, under your /loop request and your go-ahead: **Contour Paint**, in
**Stylize**, after the Contour Paint effect in the After Effects beta. It smears the picture's
colours along its own edges, so it looks painted in long strokes that follow the shapes, and can
lay flat poster colours over it.

Contour Paint is only in Adobe's beta so far, and Adobe has published its controls but none of
their values or how it works inside, so every number and the painting itself are our own, and it
is not matched to After Effects pixel for pixel.

- **View** (Final Result): Edge Map shows the edges' strength in grey with short cyan lines along
  the way the strokes run.
- **Stylization** (50, 0 to 100): how long the strokes are.
- **Cleanliness** (50, 0 to 100): how much small detail the strokes ignore.
- **Smear Iterations** (3, 0 to 100): how many times the smear runs.
- **Apply to Alpha** (Off): On smears the see-through parts too.
- **Draw Color Fills** (None): Over Smeared Image or Only lays flat colours; **Reduction Method**
  (K-Means or Median) and **Posterization Level** (8, 2 to 32) pick them; **Low** and **High
  Strength** (0 and 100) limit them to edges of that strength.
- **Offset Flow Angles** (0): turns the strokes. **Quantize Flow Angles** (0, 0 to 90): snaps them
  to steps of that many degrees.
- **External Edge Map** (none): another layer whose edges steer the strokes.

Not built yet (Adobe has not published enough): Match to Nearest Custom Color, Freehand Flow
Adjustments, and the arrowheads on the Edge Map view.

The check, `verification/D-470_contour_paint_table.md` (194 of 194), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. With Quantize Flow Angles above 0 the processor draws it: a
snapped stroke direction is an all-or-nothing step, which the card would tip differently at a few
pixels. The pictures are in `verification/D-470 pictures/`: the street (`town.png`,
`1_before.png` without the effect).

## What to check

1. **As added.** `2_as_added.png`: the street lightly smeared along its edges (look at the road's
   dashes and the window rows), the same shape and see-through parts as before.
2. **Longer strokes.** `3_long_strokes.png`: Stylization 100 and 6 smears: plainly longer strokes.
3. **Poster colours.** `4_fills_6_colours.png`: the street in at most 6 flat colours.
4. **Fills on the edges only.** `5_fills_only_strong_edges.png` (the pink stands for see-through):
   flat colours only along the buildings' and road's edges, see-through in between.
5. **Edge Map.** `6_edge_map.png`: dark, with the edges in grey and short cyan lines along them.
6. **Steered by another layer.** `7_steered_by_bricks.png`: with the brick wall (`brick_wall.png`) as
   the External Edge Map, the street is smeared sideways along the bricks' lines instead of its own.
7. **In the app.** Add Contour Paint (Stylize) to a picture; drag Stylization and Smear Iterations;
   switch Draw Color Fills to Over Smeared Image and change Posterization Level; turn Offset Flow
   Angles; pick a layer under External Edge Map; switch View to Edge Map.
8. **Out of range.** Type 120 in Stylization: it is refused with a sentence giving the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-350_contour_paint_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Contour Paint as added on each of three layers, played again, 97.7 ms a frame
on the card against 17.6 for the Noise alone; on the processor 674.1 against
42.8: a busy machine cannot say whether the card is quicker than the processor for this effect.
