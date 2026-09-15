# W-26: duplicate and delete a composition, shy layers, sequence layers, preferences

Asked for on 2026-09-15: "proceed", on the batch proposed after the W-25 playtest.

No photograph: every item is a click, and the capture script cannot click.

## What to check by hand

Open `verification/W-16_key_shapes_project.json`:

1. **Duplicate a composition.** In the Project panel, each composition row has two new buttons at its right: a double square and a cross. Press the double square: a composition named the same with "copy" on the end appears and is on screen, with the same layers, keys, effects, mattes, work area and markers. Scrub it: it looks the same as the original. Ctrl+Z: the copy is gone and the original is on screen.
2. **Edit the copy, not the original.** Duplicate again, move a layer in the copy, then click the original's row: the original is unchanged.
3. **Delete a composition.** Press the cross on the copy's row: it disappears, the window shows the first composition left, and the strip says "... is deleted. Ctrl+Z brings it back." Ctrl+Z: it is back, in its place in the list, with its layers.
4. **The last one stays.** In a project with one composition, press its cross: nothing is deleted, and the strip says it is the only composition.
5. **Shy.** Each layer row on the timeline has a small half-circle switch after the lock. Press it on two layers: the row says "shy". Press Hide shy layers at the top of the timeline: those two rows leave the list, and the picture still shows them. Press it again: they are back. One Ctrl+Z per shy press. Close and reopen the window: Hide shy layers is still how you left it. Save and reopen the project: the layers are still shy.
6. **Sequence layers.** Click layer 1, then Shift+click layer 3 (all three chosen), or Ctrl+click layers one at a time. Right-click one of them and choose Sequence layers: the top chosen row stays put, the next row down starts where it ends, the next where that one ends, each as long as before. One Ctrl+Z puts them all back. With only one layer chosen the line is greyed out.
   - Playtest fix: the layer names had been squeezed to "l.." by the new shy switch; the name column is wider now and the names read in full.
7. **Preferences.** Press Preferences... on the menu bar: a box opens in the Project panel. Set the width to 1280, the height to 720 and the length to 48. Press New composition...: its form now starts at 1280, 720 and 48. Drag a panel border somewhere odd, open Preferences and press Put the panel sizes back: the panels return to their first sizes. Close and reopen the window: the preferences are kept. Open a different project: they are the same, as they belong to the window.
8. **Palette.** Press Ctrl+Shift+P anywhere in the window (not while typing in a box): a search box opens in the middle. Type "shy", "sequence", "duplicate", "delete the composition" or "preferences": each has a line.

## What checks it by machine

- `verification/B-05_model_table.md`: the shy switch applies and undoes, deleting the only composition is refused, deleting one of two leaves the other, and undo puts both back in their places.
- `verification/B-12b_command_map_table.md`: `composition.duplicate`, `composition.delete` and `layer.toggle_shy` are answered.
- `verification/B-12c_keyboard_table.md`: the Preferences, Close, Put the panel sizes back and Hide shy layers buttons are controls.
- **Not** checked by a test: that the copy matches the original on screen, which rows Hide shy layers leaves out, where Sequence layers puts each layer, and the preferences box. Steps 1 to 8 are the check.
- New capabilities the owner may cut: composition duplicate and delete, the shy switch, sequence layers.
