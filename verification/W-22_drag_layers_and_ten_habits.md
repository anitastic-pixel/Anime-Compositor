# W-22: drag layers and effects up and down, and more After Effects habits

Asked for on 2026-09-14: "next batch of items; also, I want the layers to be able to change their order/switch them around, but dragging them like in AE", then "I want to be able to drag/switch the layers around, along with the effects panel as well like AE".

No photograph: every item is a drag or a key press, and the capture script can do neither.

## What to check by hand

Open `verification/W-16_key_shapes_project.json`:

1. **Drag a layer.** Press on a layer's name in the timeline list and drag it up past another layer, then let go. The row follows the pointer and the picture changes stacking when you let go. One Ctrl+Z puts it back. A plain click on the name still just selects it.
2. **Drag an effect.** Add two effects to a layer. Press anywhere on an effect card's top bar (not on its buttons) and drag it below the other one. The order swaps. The ●, ▲, ▼ and ✕ buttons still work as clicks.
3. **Drawing onto the list.** Drag a drawing from the media bin and let go on the layer list. A blue line shows where it will land. Let go on the top half of a row: the new layer lands above that row. Let go on the bottom half: below it.
4. **Ctrl+D.** Select a layer and press Ctrl+D. A copy with the same name appears just above it. Ctrl+Z removes it.
5. **Ctrl+Shift+D.** Put the playhead in the middle of a layer and press Ctrl+Shift+D. Its bar becomes two bars in two layers, meeting at the playhead. One Ctrl+Z makes it one layer again. On the layer's first frame it says it cannot split there.
6. **Ctrl+A / Ctrl+Shift+A.** Ctrl+A highlights every layer. Ctrl+Shift+A clears the layers and any chosen keys.
7. **Alt+Shift+P.** Select a layer, move the playhead, press Alt+Shift+P. A position key appears at the playhead and the Position row opens. Pressing it again on the same frame does nothing (it does not remove the key). Alt+Shift+A, S, R and T do the same for anchor, scale, rotation and opacity.
8. **Shift snaps the playhead.** Drag along the ruler with Shift held. The playhead jumps onto keys and layer ends when it passes near them.
9. **Shift snaps to the playhead.** Drag a key with Shift held: it lands on the playhead when near it. Drag a layer bar with Shift held: its start or its end lands on the playhead.
10. **Shift+F3.** Swaps between the timeline and the graph.
11. **Clicks in the layer list.** Ctrl-click adds or removes one layer. Click one layer, then Shift-click another: every layer between them is selected.

## What checks it by machine

- `verification/B-12a_editing_table.md`: moving a layer to a place, the refusals, duplicate and split, each with its undo.
- `verification/B-12b_command_map_table.md`: `layer.move`, `layer.duplicate` and `layer.split` are in document 24, reached, and Ctrl+D and Ctrl+Shift+D are bound.
- `verification/B-12c_keyboard_table.md`: Shift+F3 is in the key list, and dragging a layer or a drawing has a keyboard way too.
- **Not** checked by a test: the drags, the snapping and the selection clicks. Steps 1 to 11 are the check.
- Splitting several selected layers at once is one undo per layer, not one for all.
