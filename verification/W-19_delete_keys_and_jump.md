# W-19: Delete removes chosen keys, J and K jump between keys

The owner asked on 2026-09-13 to "work on the next batch of items you recommend". These are two of After Effects' everyday key habits the window did not have. Before this, Delete removed the selected layer even with keys chosen, and there was no way to jump to a key except by dragging the playhead onto it.

No new photograph: both are key presses, and a still picture of a playhead on a frame cannot show that a key press put it there.

## What to check by hand

Open `verification/W-16_key_shapes_project.json`, click layer4, press `P`:

1. **Delete removes keys.** Click the position key at 40, Shift-click the one at 80, and press Delete. Both keys are gone and layer4 is still there. The Undo button reads "Undo Remove position keyframe at frame 40 and 1 more". One Ctrl+Z brings both back.
2. **Delete with nothing chosen.** Click an empty part of the timeline so no key is lit, then press Delete. The layer is deleted as before. Ctrl+Z brings it back.
3. **K and J.** With layer4 selected, press `K` a few times. The playhead lands on each of layer4's keys in turn, and the frame number beside Play shows the key's frame. `J` goes back the same way. Click away so no layer is selected: `K` then stops at every layer's keys.

## What checks it by machine

- `verification/B-12a_transform_table.md`, four new rows under W-17's: two chosen keys deleted together say so, neither is left, it is one history entry, and one undo puts both back.
- `verification/B-12c_keyboard_table.md`: J and K are now in the list of keys the window answers.
- **Not** checked by a test: that Delete chooses keys over the layer, and where J and K land. Steps 1 to 3 are the check.
