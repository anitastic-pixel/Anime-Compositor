# B-24e: a path key moved and eased, by hand

Built on 2026-09-22. Nothing new was decided for this; it lifts the limit B-24d and B-25c left:
a mask's or a shape's key could only be set, never moved, and was always linear.

The generated half is `verification/B-24e_panel_table.md`, 13 of 13, which checks what the
window writes and where the path is drawn at each frame. This sheet covers what it cannot judge:
whether the marks behave under the hand.

## Before you start

Open any project with a composition, or use **Save As...** first and work on the copy. Draw a
mask on a layer (press **Q** and drag), press the **diamond** on its timeline row at frame 0, go
to frame 20 and drag a corner of the mask somewhere else. The row now has two key marks.

## What to check

1. **Choosing a key.** Press the mark at frame 20. It lights up as a chosen property key does, and
   the playhead goes to frame 20.
2. **Dragging a key.** Drag the mark at frame 20 left to about frame 10 and let go. The mark stays
   where you let go. Play or scrub: the mask now arrives at its second shape at frame 10, twice as
   fast as before.
3. **Undo.** Press **Ctrl+Z** once. The key is back at frame 20.
4. **Easing.** Press the mark at frame 0 and press **F9**. The mark's right half changes to the
   hourglass shape of an eased key. Scrub from 0 to 20: the mask starts slowly and speeds up, where
   before it moved at an even pace.
5. **Easing both sides.** Add a third key at frame 40 (go there, drag a point). Press the middle
   mark and press **F9**: both halves of the middle mark change shape, and the mask slows down into
   frame 20 and starts slowly out of it.
6. **Holding.** Press the mark at frame 0 and press **Ctrl+Alt+G**. The mark's right half turns
   square. Scrub: the mask stays exactly still until frame 20, then jumps to the next shape. Press
   **Ctrl+Alt+G** again: it moves smoothly again (as a straight, linear move; an ease it had before
   the hold is not remembered).
7. **Dropping on a key.** Drag the mark at frame 40 onto the one at frame 20. Only one mark is left
   at frame 20, and the mask there has the shape the dragged key had.
8. **Shapes too.** Press **New shape layer**, draw a rectangle, and repeat steps 2 and 4 on its
   shape's row. It behaves the same way.
9. **Property keys still work.** Press a Position key on any layer and press F9: it eases as it
   always has, and the mask's key is no longer lit.
10. **Saved and reopened.** Save, then **Open** the same file. The keys are where you left them,
    with the same shapes, and the mask moves the same way.

## Known limits

- A path key is not in the graph editor. The graph draws a number over time, and a path is not one
  number. After Effects shows only a path's speed there.
- A path key is chosen on its own: Shift-clicking does not add a second path key, and it cannot be
  chosen together with property keys. Copy, paste, Delete and the arrow keys do not act on it.
- Right-clicking a path key opens no menu.

## Result

Pass / fail, and what was seen if it failed:

