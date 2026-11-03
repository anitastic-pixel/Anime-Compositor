# B-868 playtest: Flip the layer horizontally or vertically (D-988)

What changed: the Layer menu has two new lines, "Flip the layer horizontally" and "Flip the layer vertically", as After Effects' Layer > Transform > Flip Horizontal and Flip Vertical. The tutorial lO54j6So7dA uses them at 2:23 to make the upside-down lower half of each letter. They are in the command finder too (Ctrl+Shift+P or the finder button), without a key of their own, as in After Effects.

## Steps

1. Open any project with a drawn layer or a text layer that is not symmetrical (a word works well).
2. Select the layer. Choose Layer > Flip the layer horizontally.
   You should see the layer mirrored left to right, staying where it was (it turns about its anchor point). Its Scale now shows -100, 100.
3. Choose Layer > Flip the layer horizontally again. It is back as it was, Scale 100, 100.
4. Choose Layer > Flip the layer vertically. The layer is now upside down, mirrored top to bottom; Scale 100, -100.
5. Press Ctrl+Z once. The layer is the right way up again.
6. Select two layers and flip them. Both flip at once, and one Ctrl+Z puts both back.
7. Click an empty part of the timeline so nothing is selected and choose a Flip line. Nothing changes and the status line says "Select an unlocked layer first."

## Not right yet

- If Scale has keys, only the value at the playhead is flipped (as Fit the layer to the composition works). After Effects' own behaviour with keys was not shown in the tutorial.

## Checks behind it

- `b868_flip_layer` (app tests): 11 of 11, `verification/B-868_flip_layer_table.md`.
- A private background copy of the app: 9 of 9, `verification/B-868_private_app_table.md`.
