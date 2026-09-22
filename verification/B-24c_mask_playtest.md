# B-24c: masks in the window, by hand

Built on 2026-09-20 against D-77, which the owner accepted on 2026-09-19 ("accept D-77, proceed
with B-24b, along with shortcut for the pen tool").

The generated half is `verification/B-24c_panel_table.md`, 17 of 17. It calls what the window
calls and checks what comes back: a mask drawn is Mask 1, Mask 2 and so on; a point moved moves
that point and no other; the mode, opacity, feather, expansion and invert each go on their own;
a shape of fewer than three points, one that crosses itself, an opacity out of range, a point
that is not six numbers, a mode nobody has and a mask that is not there are each refused with
the reason; Delete takes one off and Undo brings it back with its settings. Whether the pixels a
mask keeps are right is B-24b's table, against `Fixtures/masks`. This sheet covers what neither
can judge: what you see on the picture, on the timeline and in the panel, and whether the tools
feel like tools.

## Before you start

Open any project with at least one drawing layer, or use **Save As...** first and work on the
copy. Nothing below changes the fixtures.

## What to check

1. **The tools.** Choose a layer. Press **G**. The status line says the pen is in hand and the
   pointer over the picture is a crosshair. Press **Q**: the rectangle. **Q** again: the
   ellipse. Press **V**: back to the selection tool, and the pointer is ordinary again.
   The old grid key has moved: **Alt+G** shows and hides the checkerboard, and plain **G** no
   longer does.
2. **A rectangle.** Press Q and drag a box across the middle of the layer. As you drag, a yellow
   outline follows the pointer. Let go: the layer is cut to that box — everything outside it is
   gone from the viewer — the tool goes back to the selection tool, and a row named **Mask 1**
   appears under the layer on the timeline reading "add, 4 points". One Ctrl+Z takes the whole
   mask back; Ctrl+Shift+Z brings it back.
3. **An ellipse.** Press Q twice and drag. The outline is a proper ellipse, not a diamond, while
   you drag and after you let go. The layer is cut to it.
4. **The pen.** Press G and click five or six places around the layer. Each click leaves a point
   and the status line says what to do next. Click the **first** point again: the shape closes
   and becomes **Mask 2**. Press G, click three points, then press **Escape**: nothing is added
   and the tool is the selection tool again. Press G, click four points, press **Enter**: it
   closes, as clicking the first point does.
5. **Moving points.** With a mask drawn, its points are small yellow squares on the picture.
   Drag one: the outline follows the hand and the picture is re-cut when you let go. One drag is
   **one** Ctrl+Z. Escape in the middle of a drag puts the point back where it started.
6. **Handles.** Draw an ellipse. Each of its points has two round handles on arms. Drag one: the
   curve bends and the point stays put. This is also one Ctrl+Z per drag.
7. **Without a mouse.** Press Tab until a mask point has a white outline. The arrow keys move it
   a pixel a press, Shift with them ten. A held arrow key keeps moving it smoothly, and the
   focus stays on the point. Right-click a layer and choose **New mask**: a mask over the whole
   layer appears, which is the only shape that needs no hand.
8. **Two masks together.** Draw two rectangles that overlap. Both are outlined; the one being
   worked on is a solid line and the other is dashed. In the layer panel each has its own block.
   Set the second one's mode to **Subtract**: the overlap is cut out of the first. Try
   **Intersect** and **Difference** and watch the picture change. Set it back to **Add**.
9. **The settings.** In the layer panel, for one mask: drag **Opacity** to 50 — the masked area
   half-fades rather than the whole layer. Drag **Feather** to 20 — the edge softens on both
   sides of the line. Drag **Expansion** to 20 — the shape grows outwards; to -20 and it shrinks.
   Press **Invert**: what was kept and what was cut swap over. Press **On**: the mask stops
   cutting altogether and the row on the timeline says "off". Each of these is one Ctrl+Z.
10. **Which mask.** With two masks, press **Edit points** in the second one's block, or click
    its name on the timeline. Its points appear on the picture and the first one's go; the row
    and the button say which is being worked on.
11. **Delete.** Press **Delete** in a mask's block. It goes from the picture, the timeline and
    the panel, and the other stays. One Ctrl+Z brings it back with its mode, opacity, feather
    and expansion as they were.
12. **A sound layer.** Choose an audio layer, if the project has one. It offers no masks at all,
    and the pen on it says it is a sound layer rather than drawing anything.
13. **Saved and reopened.** Save, then **Open** the same file. Every mask is there, with its
    points, curves, mode and settings, on the right layer and in the right order.
14. **Export.** Export a few frames as PNG. The exported frames are cut exactly as the viewer
    showed them.

## Known limits

- A mask's path cannot be animated yet: there is no stopwatch on it, and the shape you draw is
  the shape at every frame. That is B-24d. A file that already holds a keyed path keeps it, is
  drawn from its base and says so in the diagnostics (B-24b). *B-24d, built on 2026-09-20,
  removed this limit: the mask's row has a stopwatch and the shape moves between its keys. See
  `verification/B-24d_mask_playtest.md`.*
- One Undo entry per change reads "Set mask of 4 points", whatever the change was: the core
  takes the whole list of masks as one command, so it names the shape and not the setting.
- The tools draw on the layer that is chosen, and only where that layer is on this frame. A
  layer outside its own in and out points has nothing to draw on and says so.
- No free-hand or rounded-rectangle tool, no feather that varies along the path, no mask
  expansion by dragging on the picture: the panel's numbers are how those are set.
- The pen places corner points; a curve is made by dragging a point's handles afterwards,
  rather than by dragging as you place it.

## Result

**Passed on 2026-09-22.** The owner walked all fourteen steps: 1 to 6 work, 7 and 8 work, 9, 10,
11 work, 12, 13 and 14 true. Nothing on this sheet did the wrong thing.

Two things were reported that no step asks about, and both are now answered in B-24g,
`verification/B-24g_drag_playtest.md`: the picture lagged considerably behind every point dragged,
and a point pressed showed no sign of having been pressed.

## What to answer

"works", or which step number did something else and what it did.
