# W-15: the timeline zooms, the graph shares its scale, and the speed graph can be pulled

Asked for on 2026-09-13: "add easing capabilities and allow interactions when messing with the speed graph, also, have the timeline scale be equivalent to the graph's all like AE; also allow scaling for the timeline itself so I can zoom in and out like in AE/Davinci resolve."

## The photographs

```
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1 -Name W-15_timeline_whole
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1 -Name W-15_timeline_zoomed -Keys "==="
```

- **`W-15_timeline_whole.png`** is the window as it opens: the ruler along the timeline runs the whole composition, and the zoom slider beside the timeline's tools is at its left end.
- **`W-15_timeline_zoomed.png`** is the same window after `=` was pressed three times. The ruler's frame numbers are closer together in frames and further apart on screen, the bars are cut where the view ends, and the slider has moved right. Nothing in the project changed, so the title bar does not say it needs saving.
- **In both, the bars start exactly under the ruler's frame 0.** The first photographs of this unit showed the ruler and the playhead about a gap's width left of the bars. That was older than this unit: each layer row had a small padding and a gap before its bar that the ruler, the playhead and the pointer did not allow for, so a click near a frame boundary could land on the frame beside it. Zooming in made it plain. The rows now have neither, and the photographs were taken again after that change.

## What no photograph here shows, and how to check it by hand

The capture script cannot click, and every one of these starts with a layer selected by a click. Open a project with a layer that has two or more keys on one property, select that layer, and:

1. **Zoom with the mouse.** Hold Alt (or Ctrl) and turn the wheel over the timeline: it zooms about the frame under the pointer, which stays put. Shift and the wheel slides the view along. `-` zooms back out; all the way out is the whole composition again.
2. **The same scale in both tabs.** Put the playhead on a key, then switch between Sheet and Graph. The playhead, the ruler numbers along the graph's top, and the key's dot are in the same column in both, zoomed or not.
3. **The speed graph.** On the Graph tab press **Speed**. The line is now how fast the property changes, per second. Each key on an eased or linear segment has a level handle: drag it sideways to change the influence (how long the ease lasts) and up or down to change the speed. Press Speed again for the value graph; the curve has changed to match. Ctrl+Z undoes a pull as one step.
4. **F9.** Click a key on the timeline: the playhead jumps onto it. Press F9 and both segments either side of the key become easy ease (the speed graph shows the speed falling to zero at the key). Shift+F9 eases only the side arriving at the key, Ctrl+Shift+F9 only the side leaving it. One Ctrl+Z undoes the whole press, however many layers were selected.

## What checks it by machine

- `verification/B-12b_command_map_table.md`: document 24 now gives `keyframe.set_interp` the shortcut F9, and F9 is bound.
- `verification/B-12c_keyboard_table.md`: the page's controls now include `graphmode` and `timezoom`, and the keys it answers include `=`, `-` and `F9`.
- **Not** checked by a test: the arithmetic that turns a speed handle's position into the ease's four numbers. It was checked once, outside the test suite, by building handles from a curve and turning them back into that same curve, and by a linear segment reading its plain rate (120 units a second for 120 over 24 frames at 24 fps). The value the file ends up holding is still what the core renders, so a wrong handle shows up as a wrong curve in step 3, not as a wrong picture that looks right.
