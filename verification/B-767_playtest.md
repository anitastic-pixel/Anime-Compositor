# B-767 playtest: a layer's bar follows the hand (D-887)

## What changed for you

Dragging a layer's bar on the timeline (to move it, or by its start or end to trim it) now moves
the bar the moment the hand moves. Before, each step waited for the project to be read back and
all the panels to be drawn again, so on a shot with many layers the bar slid behind the hand,
smoothly but late. The viewer still follows the drag and shows the frame for the newest step. The
panels are drawn again once, when you let go. Nothing about the project, the undo list or the
pictures changed: a drag is still one undo entry, and Escape still puts it back.

On this machine, with 25 layers, the bar now trails the hand by under 17 ms (one screen frame)
where it trailed by 0.2 to over 1 second, and the viewer by 23 to 59 ms where it trailed by about
150 ms (`verification/B-767_timing.md`).

Not changed: dragging the seam between two exposures on a drawing's bar still waits for each step,
because each step starts from where the last one left the seam. Key diamonds and the playhead
were already drawn in the page and are unchanged.

## What to look at

- `verification/B-767_bar_drag_table.md`: 15 of 15 checks pass. The frames after a drag are the
  frames of the bar put there in one step, undo and Escape put the bar and the frames back, and
  a drag is one undo entry.
- `verification/B-767_timing.md`: the before and after delays.

## Try it

1. Open a project with many layers. Drag a layer's bar left and right quickly.
2. The bar should stay under the pointer, with no sliding behind. The viewer should follow.
3. Let go. The bar stays where you put it; the panels redraw once.
4. Press Ctrl+Z: the bar goes back to where it was, in one step.
5. Drag a bar again and press Escape before letting go: it jumps back and nothing is added to undo.
6. Drag a bar's start or end edge to trim it: it should follow the pointer the same way.
