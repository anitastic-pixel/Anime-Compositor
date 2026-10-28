# B-828 playtest: all sixteen label colours (D-948)

What changed: the label colours are After Effects' sixteen, in its order and with its names. The first eight look as before.

## Steps

1. Open a project you saved before this build that has coloured layers. Each layer keeps the colour it had.
2. Right-click a layer's row. The label squares are two rows of nine: "no label" and red to blue on the first row, then green, purple, orange, brown, fuchsia, cyan, sandstone and dark green. Hover a square to read its name.
3. Click the orange square. The layer's row square and its bar turn orange.
4. Choose three layers and press **Ctrl+Shift+P**, type "dark green" and pick "Label the selected layers dark green": all three turn dark green.
5. Right-click a composition in the Project panel: the same sixteen squares; pick purple, and its dot turns purple.
6. Save, close and open the project again: every new colour is still there.
7. **Ctrl+Z** takes a label back.

## What you should not see

- A layer you coloured before this build showing a different colour.
- The label squares running off the side of the menu.

The computer's checks are in `verification/B-828_label_colours_table.md`; the check in a private copy of the app is `verification/B-828_private_app_table.md`.
