# B-322: Scribble

Built on 2026-10-10 as D-442, under your /loop request: After Effects' Scribble, in
**Generate**. It fills a closed mask with zigzag lines, like hatching drawn by hand, and can make
them wiggle as the shot plays. The layer needs a mask: draw one with the Pen or a shape tool and
set the mask's mode to None if you do not want it to cut the layer.

- **Scribble** (Single Mask): fill one mask (the one numbered in **Mask**), every mask in turn
  (All Masks), or all masks combined by their own modes (All Masks Using Modes: Add, Subtract,
  Intersect, Difference, and inverted).
- **Fill Type** (Inside): fill the inside, or only a band along the outline: Centered Edge (on
  the line), Inside Edge, Outside Edge, Left Edge or Right Edge (which side depends on the way the
  mask was drawn). **Edge Width** (10) is how wide the band is.
- **Color** (white), **Opacity** (100), **Angle** (45: the way the lines run), **Stroke Width**
  (5: how thick they are).
- **Curviness** (5): how round the turns at the ends of the lines are; 100 makes loops.
- **Spacing** (5): how far apart the lines are. **Path Overlap** (0): how far each line runs past
  the edge (below 0 they stop short). Each has a **Variation** that makes the lines uneven.
- **Start** and **End** (0 and 100): how much of the scribble is drawn; animate End from 0 to 100
  to draw the scribble on. **Fill Paths Sequentially** (on): with several masks, one after another
  rather than all at once.
- **Wiggle Type** (Smooth) and **Wiggles/Second** (5): Static never changes; Jumpy makes a new
  scribble 5 times a second; Smooth glides from one to the next. **Random Seed** (1) gives a
  different scribble.
- **Composite** (On Original Image): *On Original Image* draws over the layer; *On Transparent*
  shows the lines alone; *Reveal Original Image* shows the layer only through the lines.

After Effects publishes no formula or starting values for this effect, so how each pixel is
worked out is our own rule, written down in document 21. The brush is Path Stroke's round one. A
draft draws at half size.

The check, `verification/D-442_scribble_table.md` (392 of 392), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-442 pictures/`
(`1_before.png` is the street with nothing on it; every picture has a star-shaped mask of mode
None on the street, which by itself changes nothing).

One thing needs your decision: two of the test cases use a mask of only two points. Their pixels
match, but the program also warns that a two-point mask encloses nothing (it warns this for every
such mask, as it did for Fill). The cases expect no warning. Proposed: the cases take the
warning. Until you decide, they are marked "in dispute" and left as written.

## What to check

1. **As added.** `2_as_added.png`: the star filled with thick white diagonal strokes, slightly
   rounded at the ends, rising to the right; the street around the star untouched.
2. **A loose hatch.** `3_red_loose.png`: thin red lines further apart over the star, uneven, with
   round loops past the star's edge at the ends, like a quick sketch.
3. **The outline only, on transparent.** `4_outline_transparent.png`: the street gone; thin yellow
   horizontal lines only in a band along the star's outline, the middle of the star empty.
4. **Revealing, half drawn.** `5_reveal_half.png` (one second into two): everything gone except
   the street seen through thick diagonal strokes covering about the upper-left half of the star.
5. **In the app.** Draw a closed mask on a layer and set its mode to None. Add Scribble
   (Generate): white strokes fill the mask and wiggle as you play. Set Wiggle Type to Static: they
   hold still; Jumpy: they jump. Key End from 0 to 100 over a second: the scribble draws on. Try
   Fill Type Centered Edge, Curviness 100, Spacing 20, and On Transparent and Reveal. Delete the
   mask: the layer is shown plain and a warning says Scribble has no mask to fill. Switch to
   Draft: the same picture.
6. **Out of range.** Type 0.5 in Spacing: it is refused with a sentence saying it runs from 1 to
   1000.
7. **Saved and opened again.** Save, close and open the project: the same settings, keys and
   picture.

## Speed

Measured on a quiet machine
(`verification/B-322_scribble_timing_table.md`). Three Scribbles on a 1920 by 1080 shot, card / processor, milliseconds a
frame played again: Noise alone 12.0 / 41.3 ms a frame, as added 94.9 / 131.0, stroke 12, spacing 30, curviness 60, Jumpy 28.5 / 58.0, Centered Edge 80, spacing 2 80.0 / 59.2. That is slower than our target (an effect should add 4 ms or less): with
dense settings playback may stutter. The cause is known (the drawing step shared with Path
Stroke checks every line for every pixel in a strip) and a fix would be its own piece of work.
