# B-848 playtest: double-click the shape tool for a mask the size of the layer (D-968)

What changed: double-clicking the shape tool in the toolbar gives the selected layer a mask
that covers the whole layer, as in After Effects.

## Steps

1. Open a composition with a picture layer and select the layer.
2. Pick the Ellipse tool (press Q until the ellipse shows). Double-click the shape button.
3. Press Ctrl+Z.
4. Pick the Selection tool (V). Double-click the shape button.
5. Press Ctrl+Z.
6. Click an empty part of the timeline so no layer is selected, then double-click the shape button.

## What to see

- Step 2: one new mask, an oval touching all four edges of the layer. The Selection tool is in hand with the
  new mask chosen, as after drawing a mask.
- Step 3: the mask is gone in one undo.
- Step 4: one new mask, a rectangle on the layer's four corners (the layer looks unchanged
  because the mask covers all of it; the mask's outline runs round the edge).
- Step 5: gone in one undo.
- Step 6: no mask is added; the bottom status line says "Select a layer first: the mask covers
  the whole of it."

`B-848_private_app_table.md` has these checks run in a private copy of the app: 9 of 9 pass.

If any of these is not so, B-848 is wrong.
