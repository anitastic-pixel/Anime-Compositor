# B-24k: the free transform box around a mask or a shape, by hand

Built on 2026-09-23 against D-81, which the owner accepted the same day ("proceed").

The box is worked out by the page itself, and no test here can run the page's arithmetic, so
this sheet is the only check on whether it scales, turns and moves correctly. The generated half
is three rows at the end of `verification/B-24h_panel_table.md`, now 23 of 23. They check what
lies underneath the box: one drag of it on a moving path counts as one thing to undo, it lands
on its key as the last step sent it, and one Undo puts the shape back.

## Before you start

Open any project with at least one layer, or use **Save As...** first and work on the copy.
Choose a layer. Press **Q** and drag out a rectangle mask on the picture, then press **V** for
the selection tool. Nothing below changes the fixtures.

## What to check

1. **Opening it.** Double-click the mask's outline. A thin yellow box appears tight around the
   mask's points. It has eight small yellow squares, one at each corner and one halfway along
   each side, and a cross-hair (the anchor) at its centre. Every point is lit. The status line
   says what each part of the box does. Double-clicking on one of the mask's points does the
   same.
2. **Moving.** Drag from anywhere inside the box. The whole mask and the box travel with the
   hand, and so does the cross-hair. With Shift held, the mask moves only across or only up and
   down. Let go and press Ctrl+Z once: the mask is back where it was.
3. **Scaling from a corner.** Hover a corner square: the pointer becomes a diagonal double
   arrow. Drag it outwards. The opposite corner stays exactly where it is and the mask grows
   towards the hand. Hold Shift while dragging: the mask keeps its proportions. Carry on dragging
   past the opposite corner: the mask flips over, as in After Effects.
4. **Scaling from a side.** Drag the square in the middle of the right-hand side. The mask
   widens or narrows only, and the left side does not move. The top and bottom squares do the
   same up and down.
5. **Scaling about the anchor.** Hold Ctrl and drag a corner square. The mask now grows around
   the cross-hair, both sides moving at once, instead of against the opposite corner.
6. **Turning.** Move the pointer just outside the box, a little past a corner. The pointer
   becomes a curved arrow. Drag round: the mask turns about the cross-hair, and the box turns
   with it rather than growing into a larger upright box. Hold Shift: it turns in steps of 45
   degrees. After a turn, a scale (steps 3 to 5) works along the turned box's own sides.
7. **Moving the anchor.** Drag the cross-hair somewhere else, for example onto a corner of the
   mask. Nothing else moves, and Ctrl+Z has nothing new to take back for it. Now turn the mask as
   in step 6: it swings about the cross-hair's new place.
8. **Taking a drag back.** Start any drag from steps 2 to 7 and press Escape before letting go.
   The mask, the box and the cross-hair go back to how they were at the start of that drag, and
   the box stays up.
9. **Curves survive.** Put the box away (step 11) and, with the pen (**G**), drag out of one
   corner so it becomes a smooth, curved point. Press V, double-click the outline, then scale
   and turn the mask. The curve scales and turns with it, keeping its shape, and that point stays
   smooth: pulling one of its handles afterwards still swings the other one opposite it.
10. **Only some points.** Put the box away. Shift-click two or three of the mask's points to
    choose them, then press **Ctrl+T**. The box goes only around those points, and dragging it
    moves, scales or turns only them; the rest of the mask stays put. With one point chosen or
    none, Ctrl+T puts the box around the whole mask instead.
11. **Putting it away.** Each of these closes the box: Enter; Escape (when no drag is running);
    a double-click away from the box; a click well away from it, which then also does whatever
    that click would normally do; pressing G for the pen; choosing another layer; choosing
    another mask. Choosing the first layer or mask again does not bring the box back.
12. **On a moving mask.** Turn on the mask path's stopwatch on the timeline, move the playhead
    a few frames on, and turn the mask with the box. A new path key appears at that frame, and
    the key at the first frame still holds the old shape. Play it: the mask swings between them.
13. **On a shape layer.** Press **New shape layer**, press **Q** and drag out a rectangle, then
    press V and repeat steps 1, 3 and 6 on its outline. It behaves the same way.
14. **Undo.** Every drag in steps 2 to 6, and in 10, 12 and 13, is taken back by exactly one
    Ctrl+Z, and Ctrl+Shift+Z puts it back again.

## Known limits

- Skew and perspective are not provided; D-81 includes move, scale and turn only.
- ~~The box holds one mask at a time: it cannot go around several masks at once~~, or around a
  whole layer. The layer's own handles do the whole-layer job. *(Several masks: lifted by
  D-81a, `verification/B-24l_box_limits_playtest.md`.)*
- ~~The box has no keyboard keys of its own.~~ The arrow keys and **[** and **]** on the chosen
  points still work as B-24i and B-24j built them. *(Lifted by D-81a.)*
- If every point in the box lies on one straight line, the box has no width (or no height) in
  that direction, so a square pulled that way does not scale anything.
- ~~Undoing a turn puts the mask back but leaves the box at its turned angle, so it sits looser
  around the mask until it is closed and opened again.~~ *(Lifted by D-81a.)*
- Whether the box is up is not saved with the project.

## Result

**Passed on 2026-09-23**, all fourteen steps, the owner answering "works; work on limits". The
second half of that answer is D-81a and B-24l.

## What to answer

"works", or which step number did something else and what it did.
