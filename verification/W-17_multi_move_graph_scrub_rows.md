# W-17: chosen keys dragged together, a steadier and livelier graph, AE-style number dragging, roomier rows

Asked for on 2026-09-13, after playing with W-16: "can't slide all selected across the timeline", "lines seem to shift up/down depending on how the timeline pans", "compress the values and give more space between the layer name and layer bars/timeline", "more polishing on how realtime it previews/influences/looks", "the graph easing circle influences need snapping when pressing shift", and "limit the majority of manipulated values to at least be in the hundredths, shift mouse drag to jump by tens or something high, and crl mouse drag for tiny value changes like AE".

## The photograph

```
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1 -Name W-17_rows -Open verification/W-16_key_shapes_project.json
```

**`W-17_rows.png`**: the layer names now have a wider column (260 pixels, was 190) before the bars start. Compare with `W-16_key_shapes.png`: the bars begin further right and the key shapes are unchanged.

## What no photograph here shows, and how to check it by hand

The capture script cannot click. Open `verification/W-16_key_shapes_project.json`, click layer4, press `P`:

1. **Dragging several keys.** Click the position key at 40, Shift-click the one at 80, then drag either one right. Both follow the pointer, keeping their distance, and stay where they are put. One Ctrl+Z puts both back. Drag a key that is *not* chosen: it goes alone and becomes the only chosen one. Drag the pair so one would land on the key at 120: nothing moves and the status line says there is already a key there.
2. **The same on the graph.** Graph tab, choose two dots, drag one: both move.
3. **The graph holds still.** Press `=` a few times and drag the scroll bar: the curve slides sideways but its height and the two numbers on the left stay the same.
4. **Live curve.** Pull an ease handle (or a speed handle with Speed on): the line bends as you pull, not only when you let go.
5. **Shift snaps influence.** Hold Shift while pulling a handle sideways: it jumps in 5% steps of the segment.
6. **Numbers.** Drag the position number on the row: it counts in hundredths at most. Hold Shift while dragging: ten times faster. Hold Ctrl: a tenth as fast. The arrow keys do the same with Shift and Ctrl. An exposure's frame numbers still move by whole frames.
7. **Rows.** The numbers beside a property's name are smaller type, with room between them and the bar.

## What checks it by machine

- `verification/B-12a_transform_table.md`, five new rows under W-11: two neighbouring keys moved together land two frames on (the one ahead moves first, so they never collide with each other), it is one entry to undo, and a move onto a key that is not moving is refused with nothing moved.
- `verification/B-12c_keyboard_table.md`: the arrow keys on a number are still the keyboard's way to do what a drag does.
- **Not** checked by a test: the graph's height, the live preview, snapping and the drag pace. Steps 3 to 6 are the check. The preview of a position segment whose path curves is drawn straight until the handle is let go, then the real curve replaces it.
