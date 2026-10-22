# B-615 playtest: every key travels with its layer (D-740)

## What to try

1. Open a project with an animated shape layer, for example one whose groups move or turn, or whose path, colour, trim or gradient is keyed.
2. Drag the layer's bar along the timeline. The animation goes with the bar: what played at the bar's first frame still plays at its first frame.
3. Put the playhead somewhere else and press **[** with the layer selected. The layer starts at the playhead and its animation starts there too. **]** does the same with the layer's end.
4. Do the same with a layer that has an animated mask, and with a sound layer whose level or pan is keyed. Their keys, in the timeline rows, move with the bar.
5. Undo puts the layer and every key back.

Before this build, only the position, scale, rotation, opacity, anchor, depth, Time Remap and effect keys moved. A shape's, a mask's and a sound layer's keys stayed where they were, so the animation came apart from its own bar.

## What the check found (private copy, `Fixtures/shape_groups/host.json`): 10 of 10 pass

- The fixture's groups move and turn over frames 0 to 24.
- **[** with the playhead at frame 10 moved the layer to start at 10. The groups' keys moved to 10 and 34. Frame 10 now draws exactly what frame 0 drew, and frame 22 what frame 12 drew, pixel for pixel.
- The bar dragged to start at 5 (the page sends what a dragged bar sends): the keys are at 5 and 29, and frame 17 draws exactly what frame 12 drew.
- Two undos gave the layer and its keys back as in the file, and frame 12 is the fixture again, pixel for pixel.
- No page errors.

Fixture check `tests/b615_layer_shift.rs`: 72 of 72 (see `B-615_layer_shift_table.md`). FX-LSHIFT-001 to 018 move 18 keyed layers from projects across `Fixtures/`, holding every kind of key there is: a sound layer's level and pan; a mask's path, feather and expansion; a shape's path, fill and stroke colour and opacity, stroke width, fill and stroke gradient points, rectangle and star settings and trim; a group's transform, repeater and path operations; and the layer's transform, position split in two, depth, Time Remap and effect settings. Each layer's keys are all moved by the same number of frames as its bar. Nothing else saved changed but the in and out points, and one undo gives the project back as saved. Before the fix, 32 of those 72 checks failed, every one on a kind listed in D-740. The model, mask, sound, Time Remap, effect key, frame blending and B-609 to B-614 shape checks still pass, and so do the app's tests.

## Pictures (`B-615 pictures/`)

- `all.png`: every picture below on one sheet.
- `00_before_f00.png`, `00_before_f12.png`, `00_before_f24.png`: the fixture as it opens, at frames 0, 12 and 24. The square turns and its gradient turns with it; the ring swings round it.
- `01_moved_f10.png`, `01_moved_f22.png`: after **[** at frame 10, frames 10 and 22, the same as frames 0 and 12 before.
- `02_dragged_f17.png`: after the bar is dragged to start at 5, frame 17, the same as frame 12 before.

## How this differs from After Effects

- None known. After Effects moves every key of a layer with its bar.
- Trimming a layer's start still leaves its keys where they are, as in After Effects, except on a stretched layer (D-216), where they move so they play where they did. Only drawings and compositions can be stretched, so this matters for none of the kinds above but masks, which it already moved.
