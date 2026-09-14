# W-20: ten more After Effects habits for the playhead, layers and keys

Asked for on 2026-09-13: "what is the next batch of items you recommend we tackle? pick 10 items", then "then run it". The agent chose these ten. Each one checked first that the window did not already have it. Key tooltips on the timeline already existed, so the tenth item gives the graph's dots the same tooltips.

No photograph: every item is a key press, a drag or a hover, and the capture script can do none of them.

## What to check by hand

Open `verification/W-16_key_shapes_project.json`, click layer4, press `P`:

1. **Home / End.** The playhead goes to the first frame, then to the last.
2. **Page Up / Page Down.** One frame back and forward. Hold Shift with them, or with the left and right arrows: ten frames.
3. **I / O.** With layer4 selected, the playhead goes to the layer's first frame, then to its last.
4. **Ctrl+Up / Ctrl+Down.** The selected layer becomes the one above it in the list, then the one below.
5. **Alt+Left / Alt+Right.** Choose a key and press Alt+Right: it moves one frame later. With Shift as well: ten. Ctrl+Z puts it back.
6. **Box select.** Press on an empty part of the position row, away from any key, and drag across two or three keys: a blue box follows and the keys inside it light up. Drag again with Shift held to add more. A click on the row without dragging still clears the chosen keys.
7. **The property's name.** Click the word "Position" on the timeline row: every position key lights up.
8. **Copy and paste.** Choose the keys at 40 and 80 and press Ctrl+C. The status line says "2 keys copied". Move the playhead to frame 150 and press Ctrl+V: keys appear at 150 and 190 with the same values and the same key shapes. One Ctrl+Z removes both.
9. **Hold.** Choose a key and press Ctrl+Alt+G (After Effects uses Ctrl+Alt+H; moved to G on 2026-09-14 at the owner's request, "change ... to a different command like crl alt g"): its shape turns square and the motion after it jumps instead of gliding. Press it again: it goes back to linear. If the key was eased, that ease is not brought back.
10. **Graph tooltips.** On the Graph tab, hover a key dot: it says the property, the frame and the value.

## What checks it by machine

- `verification/B-12c_keyboard_table.md`: the new keys are in the list of keys the window answers, and the existing arrow, Delete and Space rows still hold.
- **Not** checked by a test: everything in the list above. It is all page behaviour built from routes the tests already cover (`property.set_base`, `keyframe.set_interp`, `keyframe.set_path`, `keyframe.move`), and steps 1 to 10 are the check.
- Paste puts keys back on the layer they were copied from, not onto a different selected layer. Copying motion from one layer to another is a later unit if wanted.
