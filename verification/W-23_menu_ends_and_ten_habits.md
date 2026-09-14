# W-23: a right-click menu, front and back, and more After Effects habits

Asked for on 2026-09-14: "ask for a playtest for current and this next batch; also, create a auto-rebuild/updater that automatically closes and reopens the app when we make our changes."

No photograph: every item is a click or a key press, and the capture script can do neither.

## What to check by hand

Open `verification/W-16_key_shapes_project.json`:

1. **Numbers.** Every row in the layer list starts with its number. The front layer is 1.
2. **Ctrl+Shift+] and Ctrl+Shift+[.** Select the back layer, press Ctrl+Shift+]. It is now row 1 and in front on the picture. Ctrl+Shift+[ sends it to the back. One Ctrl+Z each.
3. **Right-click a layer.** A menu opens with Rename, Duplicate, Split at playhead, Bring to front, Send to back and Delete, each with its key written beside it. Clicking a line does that thing. Clicking anywhere else, or Escape, closes it without doing anything. Right-clicking a row that is not selected selects it first.
4. **Enter.** Click a layer's name, then press Enter: the name becomes a text box, as F2 does. Escape cancels it.
5. **Escape.** With layers selected and keys chosen, press Escape (when no name box is open). Nothing is selected any more.
6. **Comma and full stop.** `.` zooms the picture in, `,` zooms it out, about the middle of what you see.
7. **Middle-button drag.** Zoom in until the picture is bigger than its area, then drag with the middle mouse button (the wheel). The picture slides about. Nothing moves in the composition, and there is nothing to undo.
8. **Alt+PageUp / Alt+PageDown.** Select a layer and press Alt+PageDown. Its bar moves one frame later; the playhead does not move. Alt+Shift+PageDown moves ten. One Ctrl+Z puts it back. Plain PageDown still steps the playhead.
9. **Shift on the picture.** Drag a layer on the picture with Shift held and wander diagonally: it only moves along one line, whichever you have gone further along.
10. **Alt-click a diamond.** On a property with several keys, Alt-click the diamond at the left of its row. Every key on that property disappears. One Ctrl+Z brings them all back. A plain click still adds or removes the one key at the playhead.

## The rebuild watcher

Open a second terminal in the project folder and run:

    powershell -ExecutionPolicy Bypass -File tools/watch_rebuild.ps1

It opens the window on your most recent project if it is not open already. Then, whenever a file under `app/` or `src/` changes and has been quiet for four seconds, it closes the window, builds, and opens the window again on the same project. If the build fails it prints the errors and opens the old window. Ctrl+C in that terminal stops it. Anything unsaved in the window is closed; the "Recover unsaved work" line brings it back.

## What checks it by machine

- `verification/B-12b_command_map_table.md`: `layer.move` is now bound to Ctrl+Shift+] and Ctrl+Shift+[ and is no longer mouse-only.
- `verification/B-12c_keyboard_table.md`: comma, full stop, Enter and Escape are in the key list; the right-click menu, the middle-button pan, the Shift-held drag and the Alt-click each have a keyboard way too.
- **Not** checked by a test: the menu, the pan, the constrained drag and the key removal. Steps 1 to 10 are the check.
- The work area (B and N) is still unbuilt: the composition has no work area in the model yet, so it is a unit of its own.
