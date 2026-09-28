# B-109b: trim paths, by hand

Built on 2026-09-27 against D-169, which is **proposed** and waits for the owner. Playing this
sheet is how to judge it: if what you see here is what you want from Trim Paths, accepting
D-169 and passing this sheet are the same answer.

The generated halves are `verification/B-109_shape_trim_table.md`, which checks the pixels
against FX-SHP-070 to 095, and `verification/B-109b_panel_table.md`, which checks what the panel
sends and what comes back. The pictures the fixtures were worked from are in
`verification/B-109a proposal/`. This sheet covers what none of them can judge: whether it looks
right on the picture and whether the panel is usable.

## Before you start

Open any project with a composition, or use **Save As...** first and work on the copy. Press
**New shape layer**, then **G** and click four or five points across the picture to draw an
open line. Under the shape, press **Add stroke** and drag its width to about 20.

## What to check

1. **Turning it on.** Under the shape, **Trim paths** says **Off**. Press it: it says **On**,
   three rows appear, **Start** 0, **End** 100 and **Offset** 0, and the picture has not
   changed, because 0 to 100 is the whole line.
2. **End.** Drag **End** down to 50. The line now stops halfway along its length, measured along
   the line and not across the picture, with a round end where it stops.
3. **Start.** Drag **Start** up to 25. The line starts a quarter of the way along. Drag Start
   past End: nothing jumps, because the lower of the two is always taken as the start.
4. **Nothing.** Set Start and End to the same number. The stroke disappears, and the shape's
   fill, if it has one, stays.
5. **Offset.** Set Start 0, End 30, and drag **Offset**. The visible piece slides along the line.
   At the far end it wraps round and carries on from the beginning, so for a moment both ends
   show a piece. 360 is one whole trip.
6. **Drawing it on.** At frame 0 set End to 0 and press its diamond. Go to frame 24, set End to
   100. Play: the line draws itself on from its first point to its last. The keys show on the
   timeline, drag, take F9 and open in the graph editor as any number's keys do.
7. **A closed shape.** Draw a rectangle with **Q**, give it a stroke, turn Trim paths on and
   animate Offset from 0 to 360. A piece of outline runs round the rectangle and comes back to
   where it started. The fill is not trimmed.
8. **Off again.** Press **On**: it says **Off**, the whole stroke is back, and **Ctrl+Z** brings
   the trim back, keys and all.
9. **Saved and reopened.** Save, then **Open** the same file. The trim and its keys are as they
   were, and nothing is reported in the diagnostics.
10. **Export.** Export the range as PNG. The frames draw the line on as the viewer showed it.

## Known limits

- D-169's "Not covered" list: the ends of a cut are always round (square and flat ends wait on
  B-110's line caps); several shapes are trimmed each on its own, never one after another as
  After Effects can; the fill is never trimmed; and the length is measured on the flattened
  outline, which on a curve differs from the true curve by a fraction of a pixel.
- The start, end and offset are numbers in the panel; there are no handles on the picture for
  them.

## Result

Accepted by the owner on 2026-09-28 without a separate run (D-175).
