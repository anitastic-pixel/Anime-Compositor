# B-236: Path Stroke along shape paths

Built on 2026-10-08 under your effects loop request, decided as D-357. This is the second part of
P0-22, "effects draw along paths". B-235 let **Path Stroke** follow a layer's masks. Now it can
also follow the paths of a **shape layer**, including open ones whose ends are not joined.

After Effects' Stroke only follows masks. To pick shapes, Path Stroke has one new setting of our
own, at the top of its controls:

- **Path From**: **Masks** (as before, and how it is added) or **Shape Paths** (the shapes drawn
  on this shape layer).

With Shape Paths, the other settings work the same way, counting shapes instead of masks. **Path**
picks a shape by number (1 is the first), **All Masks** follows every shape, and **Stroke
Sequentially** draws them one after another. A shape is followed whether or not it has a fill or
stroke of its own.

The check, `verification/D-357_stroke_shapes_table.md` (103 of 103), holds every pixel to numbers
worked out by a separate program before the code existed. It also holds the viewer's picture to
within 1 level of an export. The pictures are in `verification/D-357 pictures/`.

## What to check

Put a new shape layer over the reference shot's street (`town.png` is the street alone).
The pen gives a closed shape a fill and an open one a stroke of its own. Switch those off to
match the pictures, which show Path Stroke's line alone.

1. **A star that draws itself on.** With the pen, click out a five-pointed star on the shape layer
   (ten clicks, out and in, then close it on the first point). Add **Path Stroke** from
   **Generate**, set **Path From** to **Shape Paths**, and give it an orange colour and Brush Size 8.
   Key **End** from 0 at the first frame to 100 a second later, then play. The line should start
   at the first point you clicked and follow the star round, points and dips, until it closes
   (`star_strip.png`, one frame at a time in `star_f00.png` to `star_f24.png`).
2. **An open path.** Draw a zigzag with the pen and do not close it. Path Stroke should draw the
   zigzag only, with nothing joining its last point back to its first (`zigzag_open.png`). Close
   the same path and the joining line appears (`zigzag_closed.png`).
3. **Trimming an open path.** On the open zigzag, key End from 0 to 100. The line should run from
   the first point to the last and stop there, without wrapping round.
4. **Several shapes.** Draw a second shape on the same layer. Path 2 strokes the second one alone.
   Turn on **All Masks** and both get a line. Turn on **Stroke Sequentially** and key End: the
   first draws on, then the second.
5. **Fill kept.** Switch the star's fill back on. The line is drawn over the filled star.
6. **Masks still work.** Draw a mask on the shape layer and set Path From back to **Masks**: the
   mask is stroked and the shapes are not.
7. **No shape.** Set Path From to Shape Paths on a footage or drawing layer, or set Path to a
   number the layer has no shape for. The layer should show as it is, with a warning that the
   effect has no path. It should not be an error.
8. **Save and open.** Save, close and reopen. Path From and the keys should be as you left them.

## Not done, on purpose or for later

- **Speed** (`verification/B-236_stroke_shapes_timing_table.md`): with three circles on one 1080p
  shape layer it adds about 2.5 to 4 ms a frame, within the target.
- **On a shape layer the stroke is drawn by the processor**, not the graphics card, even with
  Draw on: GPU. This is the project's existing rule for every effect on a shape layer, not something
  new here. The pictures are the same either way.
- **Masks are still always closed.** Our masks are closed by definition. Changing that would reach
  into masks everywhere (saving, the pen, undo), so it is logged as a gap. Open paths work on shape
  layers.
- **Only the layer's own shapes.** Path Stroke cannot follow another layer's shapes.
- **No star tool.** Stars are drawn with the pen.
- **Text outlines** cannot be followed yet. They wait on the text change that is waiting for you.
- A shape's own Trim Paths does not shorten the line Path Stroke follows. Use Start and End.

## If something is wrong

Say which step. The most likely faults would be in step 1 (the line starting at the wrong point
or going the wrong way) or step 2 (an open path being closed).
