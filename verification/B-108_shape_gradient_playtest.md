# B-108b: gradients on shapes, by hand

Built on 2026-09-27 against D-168, which is **proposed** and waits for the owner. Playing this
sheet is how to judge it: if what you see here is what you want from a gradient, accepting
D-168 and passing this sheet are the same answer.

The generated halves are `verification/B-108_shape_gradient_table.md`, 31 of 31, which checks
the pixels against FX-SHP-040 to 069, and `verification/B-108b_panel_table.md`, 18 of 18, which
checks what the panel sends and what comes back. The pictures the fixtures were worked from
are in `verification/B-108a proposal/`. This sheet covers what none of them can judge: whether
it looks right on the picture and whether the panel is usable.

## Before you start

Open any project with a composition, or use **Save As...** first and work on the copy. Press
**New shape layer**, then **Q** and drag a large rectangle across the middle of the picture.

## What to check

1. **Turning it on.** In the layer's panel, under Shape 1's **Fill**, change **flat** to
   **linear**. The rectangle now runs from its grey on the **left edge** to **black** on the
   **right edge**, smoothly. Three new rows appear: **Fill stop 1**, **Fill stop 2**, and the
   **start** and **end** points with their diamonds.
2. **A stop's colour.** Pick a bright red for Fill stop 1 and a blue for Fill stop 2. The
   rectangle goes red to blue. The middle should look like After Effects' own gradient between
   those two colours: a purple that is not muddy or dark.
3. **A third stop.** Press **Add stop**. A white stop appears in the middle (at 50%). Drag its
   **at** number: the white band slides left and right. Drag its opacity down: the band fades,
   and what is under the layer shows through there.
4. **Removing.** Press **Remove** on the middle stop. It is gone and red-to-blue is back. With
   two stops left, the Remove buttons are greyed out, because a gradient needs two.
5. **Radial.** Change **linear** to **radial**. The colours now spread in **circles** out from
   the start point, red in the middle, blue from the end point's distance outward. The stops and
   points are the ones you had.
6. **Moving the points.** Type new numbers into the **end** row, or drag them. The gradient
   stretches or shrinks. Drag the start's numbers: the centre of the circles moves.
7. **Animating the points.** At frame 0 press the diamond on the **start** row. Go to frame 24
   and change the start's numbers. Scrub between: the gradient slides smoothly. The start row
   now also shows on the timeline under the layer, with its two keys, and those keys drag, take
   F9 and open in the graph editor as a position key's do.
8. **A stroke gradient.** Press **Add stroke** and drag its width to about 30. Change its
   **flat** to **linear**. The outline itself now changes colour across the frame, left to
   right, as After Effects draws a stroke gradient.
9. **Back to flat.** Change the fill's **linear** or **radial** back to **flat**. The rectangle
   is its own single colour again. **Ctrl+Z** brings the gradient back, stops and keys and all.
10. **Saved and reopened.** Save, then **Open** the same file. The gradients, their stops and
    their keys are as they were, and nothing is reported in the diagnostics.
11. **Export.** Export the range as PNG. The gradients are in the frames as the viewer showed
    them.

## Known limits

- The start and end points are set by their numbers in the panel. After Effects also draws
  two small handles on the picture to drag them; that is not built.
- D-168's "Not covered" list: no highlight on a radial gradient, the end colours carry on past
  the ends rather than repeat, a stop's colour and opacity move together, a hard step inside a
  gradient is not smoothed, and a point moves in a straight line between keys.
- A stop's own colour and position are not keyed; the start and end points are.

## Result

Accepted by the owner on 2026-09-28 without a separate run (D-175).
