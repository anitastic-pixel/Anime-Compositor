# W-18: the current frame typed or dragged, Ctrl for fine handle pulls, shorter history labels

Asked for on 2026-09-13, after playing with W-17: "have it that I can input/change the current frame as well", the Shift/Ctrl paces "didn't work out, I mean these handles" (the graph's ease handles), and "compress this info box and the bottom info boxes" (the Undo button and the status line reading "Keyframe scale at frame 142 to (1.6583, 1.6583)").

## The photograph

```
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1 -Name W-18_frame_and_labels -Open verification/W-16_key_shapes_project.json
```

**`W-18_frame_and_labels.png`**: beside Play and the arrows, "frame 0" now has its number underlined like the numbers on the rows, because it can be dragged and typed into. The sentences in the box along the bottom are in smaller type than in `W-17_rows.png`.

## What to check by hand

Open `verification/W-16_key_shapes_project.json`, click layer4, press `P`:

1. **The frame number.** Under the viewer, beside Play, the number after "frame" is now a number like the ones on the rows.
   - Drag it left and right: the picture scrubs.
   - Click it without dragging, type `50`, press Enter: the playhead goes to frame 50.
   - Click it and press the arrow keys: one frame at a time. Hold Shift for ten.
   - Type a frame past the end: the playhead stops at the last frame. Type `-5`: it goes to frame 0.
2. **Fine handle pulls.** On the Graph tab, pull an ease handle and hold Ctrl partway through the pull. The handle now moves a tenth as far as the pointer. Let go of Ctrl and it follows the pointer again.
3. **Shift snaps the angle.** Asked for after the first W-18 build: "snap to it's 90 horizontal line/angle as well, so I guess to snap in a angular way, not distance". Pull a value graph handle with Shift held: it turns in 15 degree steps, flat among them, and its length still follows the pointer. On the speed graph a handle is always flat, so there Shift still puts influence on 5% steps.
4. **Short labels.** Change scale with a drag so it lands on a long number, then look at the Undo button and the green line at the bottom. The green line is also rounded when it arrives with a frame, which the first W-18 build missed ("Keyframe scale at frame 19 to (1.9882, 1.9882)"). They read e.g. "Keyframe scale at frame 142 to (1.66, 1.66)", in smaller type. Hover over Undo: the tooltip gives the full numbers. A very long label is cut with "…".

## What checks it by machine

- `cargo test -p anime_compositor_app`: 29 pass. The keyboard and control contract tests still find every control focusable, including the new frame number (it is the same kind of number as the row values).
- **Not** checked by a test: the three steps above. They only change how the page reads a pointer and how it prints a sentence. The saved file and the undo history keep the exact numbers.
