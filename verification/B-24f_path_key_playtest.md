# B-24f: a path key handled like any other key, by hand

Built on 2026-09-22. Nothing new was decided for this. It lifts the limits B-24e left: a mask's or
a shape's key could be dragged and eased, but only on its own, and it was not in the graph editor.

The generated half is `verification/B-24f_panel_table.md`, 16 of 16. It checks what the window
writes, and what the graph is given, for each thing below. This sheet covers what it cannot judge:
whether the marks, the menu and the graph behave under the hand.

## Before you start

Open any project with a composition, or use **Save As...** first and work on the copy. Draw a
mask on a layer (press **Q** and drag). Press the **diamond** on its timeline row at frame 0, go
to frame 20 and drag a corner of the mask somewhere else. Then open the layer's **Rotation**, press
its diamond at frame 0, go to frame 20 and change the rotation. The mask's row and the Rotation row
now have two key marks each.

## What to check

1. **Choosing with other keys.** Press the mask's mark at frame 20, then **Shift**-press the
   Rotation's mark at frame 20. Both are lit.
2. **Moving them together.** Drag either lit mark to about frame 30. Both keys go to frame 30, and
   one **Ctrl+Z** puts both back.
3. **Arrow keys.** Press the mask's mark at frame 20 and press **Right arrow** three times. It
   moves one frame for each press.
4. **Dropping on a key.** Drag the mask's mark at frame 0 onto its mark at frame 20 (or wherever
   it is now). Nothing moves, and the status line says there would be a second key on that frame.
   This is how a property's key behaves, and it replaces B-24e's step 7.
5. **Right-click.** Right-click a mask mark. The key menu opens. **Easy ease** there eases it, as
   F9 does. **Key speed and influence** and the three bezier kinds are for a number's keys, so they
   leave a path's key alone.
6. **Letting a hold go.** Press the mask's mark at frame 0, press **F9**, then **Ctrl+Alt+G**: the
   mark's right half turns square. Press **Ctrl+Alt+G** again: the mark gets its hourglass back, and
   the mask starts slowly again as it did before the hold. The same is now true of a Rotation key.
7. **Copy and paste.** Press the mask's mark at frame 20 and press **Ctrl+C**. Go to frame 40 and
   press **Ctrl+V**. A third mark appears at frame 40, and the mask there has the shape it had at
   frame 20. One **Ctrl+Z** takes it away.
8. **Delete.** Press a mask mark and press **Delete**. The key is gone. Delete both marks and the
   mask stays still, keeping the shape the first of them had.
9. **The graph.** With the layer selected, press **Graph** above the timeline and choose
   **Mask 1 path** in the list. The line climbs from 0 at the first key to how far, on average, the
   mask's points moved by the second, and is level before and after. With the first key eased, the
   line starts flat and curves up. Press the speed button: it shows pixels a second, rising and
   falling between the keys. Pull a speed handle: the mask's motion changes as the handle says.
   Dragging a key's dot in the graph moves it along time only, never up or down.
10. **Shapes too.** Press **New shape layer**, draw a rectangle, key it at two frames, and repeat
    steps 1, 5 and 9 on its shape's row. It behaves the same way.

## Known limits

- A path key's value is its shape. The shape is drawn on the picture, not typed in the key menu's
  value dialog (only the frame is typed there) and not dragged up or down in the graph.
- A released hold remembers the ease it replaced only while the window is open. A key held, saved
  and opened again comes back linear when the hold is released, as before.
- The graph shows how far the path has travelled on average. When an ease overshoots and the path
  comes back, the line still goes up, because distance does not run backwards.

## Result

PASS. Run by the owner on 2026-09-23: "works". All ten steps behaved as written, the graph in
step 9 included.

