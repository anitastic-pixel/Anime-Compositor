# B-26c: null layers in the window, by hand

Built on 2026-09-23 against D-82, which the owner accepted the same day ("proceed").

The generated half is `verification/B-26c_panel_table.md`, 10 of 10. It calls what the window
calls and checks what comes back: New null needs no drawing chosen, is named Null 1, Null 2 and
so on, stands at the composition's centre, comes back off Undo, refuses a blend mode or being a
matte with a sentence, gives the picture its square to outline, and carries a layer parented to
it. Whether the frames are right is B-26b's table, against `Fixtures/null`. This sheet covers
what neither can judge: what you see in the viewer, on the timeline and in the panels.

## Before you start

Open any project with at least one drawing layer, or use **Save As...** first and work on the
copy. Nothing below changes the fixtures.

## What to check

1. **The button.** Beside **New shape layer** is **New null**. It is usable as soon as a
   composition is open, with nothing chosen under Drawings. Hovering it says what a null is.
2. **Adding one.** Choose a layer in the layer list and click **New null**. A layer named
   "Null 1" appears just above the chosen one, marked "null" at the end of its row, with no
   blend list on its row. **The picture does not change at all.** In the middle of the viewer
   there is a small square outline, and with the null chosen it has the usual orange box,
   corner handles and the anchor's cross-hair in its middle. Its bar on the timeline is a thin
   grey strip that runs the whole composition; hovering it says it is never drawn.
3. **The keys.** With nothing chosen, press **Ctrl+Alt+Shift+Y**. "Null 2" appears at the front.
   Now press **Ctrl+Alt+Y** (no Shift): an adjustment layer appears, not a null. Press Ctrl+Z
   twice: both are gone. New null is also in the command palette (Ctrl+Shift+P) and in the
   timeline's right-click menu.
4. **The square when not chosen.** Choose a drawing layer in the layer list instead.
   Each null's square stays on the picture as a faint dotted grey outline, so you can find it.
   Switch a null off with its eye: its square goes. Switch it on again.
5. **The inspector.** Choose a null. Drawing reads "none: a null, never drawn, there to be a
   parent". There are rows for Anchor, Position, Scale, Rotation, Opacity, Parent and Depth.
   There is **no** Blend list, no Masks line and no Matte row. The Effects panel says "A null is
   never drawn, so it takes no effects." and **Add effect** is greyed out.
6. **Moving it on the picture.** Drag the null's square: it follows the hand, and letting go is
   one thing to undo. Drag a corner to scale it and the turn handle to rotate it: the square
   grows and turns with it. Still nothing in the picture itself changes.
7. **The point of it: a parent.** Choose a drawing layer, and in its Parent row choose Null 1.
   The drawing does not move. Now drag Null 1's square: the drawing moves with it. Rotate the
   null: the drawing swings round the null's cross-hair. Scale the null: the drawing grows
   and shrinks about it. Parent a second drawing to the same null and move the null again: both
   go together.
8. **What it keeps to itself.** Set the null's Opacity to 10%. The drawing parented to it stays
   fully opaque: opacity does not pass to a child. Switch the null off with its eye: the drawing
   still rides on it and stays where it is.
9. **Not a matte.** Choose a drawing layer and open its Matte list. No null is in it.
10. **Saved and reopened.** Save, then **Open** the same file. The null is there, with its place,
    its name and its children still riding on it. It never shows as a missing drawing.
11. **Export.** Export a few frames as PNG. No square, no outline, nothing of the null is in
    them; the parented drawings are where the viewer showed them.

## Known limits

- The window has no Layer menu of its own: New null is a button beside New shape layer, a line
  in the timeline's right-click menu and the palette, and Ctrl+Alt+Shift+Y, where New solid
  and New shape layer already stand.
- The camera cannot be parented to a null (D-82): the camera is part of the composition, not a
  layer (D-58), so After Effects' usual camera rig is not possible yet.
- Drawing a mask on a null with the pen is refused with a sentence rather than the pen being
  greyed out.
- The square is always 100 by 100 in the null's own space; After Effects' is too, and it has no
  setting for it.

## What to answer

"works", or which step number did something else and what it did.
