# W-24: the work area, markers, solo, labels, a new project and the command palette

Asked for on 2026-09-14: "proceed with the full batch".

No photograph: every item is a click or a key press, and the capture script can do neither.

## What to check by hand

Open `verification/W-16_key_shapes_project.json`:

1. **B and N.** Put the playhead on frame 5 and press B, then on frame 15 and press N. A blue band along the top of the ruler now runs from 5 to 15. Space plays only frames 5 to 15, round and round. Ctrl+Z twice takes it back to the whole shot.
2. **Pull the ends.** Drag either end of the blue band along the ruler. It follows the hand and stops at the other end. Escape during the drag puts it back. One Ctrl+Z undoes a whole drag.
3. **Double-click the band.** It covers the whole shot again. One Ctrl+Z brings the short band back.
4. **Shift+Home and Shift+End.** With a short band, they put the playhead on its first and last frame. Plain Home and End still go to the shot's ends.
5. **Saved.** Save, close, reopen. The band and the markers from step 6 are where you left them.
6. **Markers.** Press `*` (Shift+8). A small orange flag appears on the ruler at the playhead. Drag it to another frame. Double-click it, type a name, press Enter: the name shows beside it. Ctrl+click it: it is gone. Each is one Ctrl+Z. Clicking a flag without moving puts the playhead on it.
7. **Solo.** Each layer row has a small circle button. Click it on one layer: the picture shows only that layer, and the row says "soloed". Solo a second layer: both show. Click them again to go back. It is not saved and there is nothing to undo.
8. **Label colours.** Right-click a layer. Under the menu lines is a row of coloured squares. Pick one: a square of that colour appears on the row and the layer's bar in the timeline takes its tint. The first square clears it. One Ctrl+Z each, and it is saved.
9. **Ctrl+V onto another layer.** Choose some keys on one layer, Ctrl+C, select a different layer, move the playhead, Ctrl+V. The keys land on the selected layer, the earliest on the playhead.
10. **Alt+[ and Alt+].** These were already built: they trim the selected layer's start or end to the playhead. Check they still do.
11. **Ctrl+N.** Make an edit so the title shows unsaved, then press Ctrl+N. The status line warns you. Press Ctrl+N again within three seconds: an empty project with one 1920 by 1080 composition opens. "Recover unsaved work" still brings the old edit back.
12. **Ctrl+Shift+P.** A search box opens with every command and its key. Type "marker": the list narrows. Arrow down, Enter: it runs. Escape or a click elsewhere closes it.

The recovery page asked for earlier (P-12) was already done before this batch.

## Playtest request: W-23 and W-24 together

Please run steps 1 to 10 of `verification/W-23_menu_ends_and_ten_habits.md`, then steps 1 to 12 above, in one sitting, with the rebuild watcher running if you like. For anything that does not behave as written, tell me the step number and what you saw instead.

## What checks it by machine

- `verification/B-05_model_table.md`: the work area, markers and label colour apply, refuse values outside the composition or past colour 8, and undo exactly.
- `verification/B-09_persistence_table.md`: an untouched work area still comes back out of a saved fixture unchanged.
- `verification/B-12b_command_map_table.md`: `project.new`, B, N, `*` and Ctrl+Shift+P are bound, and no command in document 24 says "nothing yet" any more.
- `verification/B-12c_keyboard_table.md`: B and `*` are in the key list, and each ruler drag and double click has a keyboard or palette way too.
- **Not** checked by a test: the look of the band, the flags, the swatches and the palette, and solo on the picture. Steps 1 to 12 are the check.
- New capabilities the owner may cut: markers, label colours, solo, the new project and the palette.
