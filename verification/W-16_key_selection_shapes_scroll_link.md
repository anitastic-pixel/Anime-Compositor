# W-16: choosing keys for F9, moving them on the graph, key shapes, the scroll bar, and linked scale

Asked for on 2026-09-13: "can't select a specific keyframe to ease it, nor can i select keyframes in the graph to move/ease ... when zooming in, I want to be able to move across the timeline when zoomed in, so I need a horizontal bar somewhere ... there is no visible change to the keyframes' look when ease is added like AE; also, let's add a default link scaling for transform tools and similar."

## The photographs

```
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1 -Name W-16_key_shapes -Open verification/W-16_key_shapes_project.json
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1 -Name W-16_key_shapes_zoomed -Open verification/W-16_key_shapes_project.json -Keys "=="
```

`W-16_key_shapes_project.json` is `B-08a_project.json` with keys added to layer4 only, so that every shape appears on its bar. The drawings it names are not beside it, which is what the orange messages at the bottom say; they have nothing to do with the keys.

**`W-16_key_shapes.png`**: layer4's bar, left to right. Each key's left half is how it is reached and its right half how it is left:

| frame | what the file says | shape to expect |
|---|---|---|
| 0 | first key, eased leaving | point on the left, hourglass half on the right |
| 20 | rotation, first key, linear leaving | diamond |
| 40 | eased arriving and eased leaving | full hourglass (bow tie) |
| 80 | linear arriving, held leaving | point on the left, square on the right |
| 100 | rotation, eased arriving, linear leaving | hourglass half on the left, point on the right |
| 120, 160 | linear | diamond |

There is no scroll bar under the rows: the timeline is not zoomed.

**`W-16_key_shapes_zoomed.png`**: after `=` twice. The same shapes, further apart, and a scroll bar under the rows whose grey thumb is a little under half its length, because a little under half the composition (107 of 240 frames) is in view.

## What no photograph here shows, and how to check it by hand

The capture script cannot click. Open `verification/W-16_key_shapes_project.json`, click layer4, press `P` to open its position row:

1. **Choosing a key.** Click the key at frame 40: it turns blue and the playhead jumps to it. Shift-click the key at 0: both are blue. Click the row away from a key: neither is.
2. **F9 on the chosen keys.** Choose the keys at 120 and 160 and press F9. Both turn into hourglasses on their eased sides; the key at 0 and 40, not chosen, do not change, wherever the playhead is. Ctrl+Z undoes it in one step. With nothing chosen, F9 does what it did before: the keys under the playhead.
3. **The graph.** Switch to Graph. Click a dot: it turns blue, and the same key is blue back on the Timeline tab. Drag a dot sideways and let go: the key has moved to that frame, on both tabs. F9 eases the chosen dots.
4. **The scroll bar.** Press `=` a few times, then drag the scroll bar's thumb: the ruler and the bars slide along together, on both tabs. Click the bar and use the arrow keys: the same.
5. **Linked scale.** In the inspector, drag Scale's first number: the second follows, keeping its ratio. Press the chain button beside Scale: it dims, and now the two change apart. On the canvas, a corner handle follows the chain; Shift while dragging does the opposite.

## What checks it by machine

- `verification/B-12c_keyboard_table.md`: the page's controls now include `timescroll`, and it is a control the Tab key stops at. The first attempt used a plain scrolling strip, which this table refused, so the bar is a slider.
- **Not** checked by a test: which shape a key gets, and the ratio arithmetic of linked scale. The photograph above is the check for the first; step 5 for the second.
