# B-849 playtest: Ctrl+T picks the Text tool; hold Space to move the view (D-969)

What changed: Ctrl+T now picks the Text tool, as in After Effects (with a mask chosen it still
opens the mask's free transform box). Holding Space and dragging in the viewer moves the view,
like the Hand tool.

## Steps

1. Select a layer that has no mask chosen and press Ctrl+T.
2. Choose a mask on a layer and press Ctrl+T, then Escape.
3. Zoom the viewer in (say 400%). Hold Space, drag in the viewer, then let go of Space.
4. Without Space, drag in the viewer with the Selection tool.
5. Tap Space once, then tap it again.

## What to see

- Step 1: the Text tool is in hand.
- Step 2: the free transform box opens round the mask, as before; Escape closes it.
- Step 3: the picture slides with the mouse. Playing may flicker on for a moment when Space goes
  down, but stops as soon as you drag, and nothing in the project moves or gets selected.
- Step 4: the view does not slide (the drag does what the Selection tool does).
- Step 5: the first tap plays, the second stops.

`B-849_private_app_table.md` has these checks run in a private copy of the app: 11 of 11 pass.

If any of these is not so, B-849 is wrong.
