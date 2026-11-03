# B-869 playtest: paste a path into Position (D-989)

What changed: as in After Effects, you can copy a mask's or a shape's path and paste it into another layer's Position, and the layer then travels along that path. The tutorial tQ5wqg10x_I does this at 1:12 to fly a rocket along a curve drawn with the pen.

## Steps

1. Open a project with a picture layer (the "rocket") and a solid or shape layer.
2. On the solid, draw a path with the pen tool (G): click four or five points across the picture, dragging on one or two of them to make curves. Leave it open or close it.
3. On the timeline, open the solid's masks (select it and press M). Press the mask's name. Press Ctrl+C.
   The status line says "Path copied".
4. Select the rocket and press P to show its Position. Put the playhead at frame 0. Press the word "Position". Press Ctrl+V.
   You should see a row of Position keys from frame 0 to two seconds later: one for each point of your path, plus one back at the first point if the path was closed. The middle keys are small circles (roving keys).
   On the picture, the rocket's motion path is the shape you drew, curves included.
5. Play it. The rocket follows the path at an even speed and stops at the last point.
6. Delete the solid's mask or the whole solid: the rocket still moves (its keys are its own now).
7. Press Ctrl+Z once: all the pasted keys go together.
8. Drag the last key further right. The whole trip takes longer, and the roving keys spread out by themselves.

## Not right yet

- Our pen does not make a separate shape-layer "Path" property as After Effects does. You copy the mask's or shape's row name instead, which is the same thing on our timeline.
- If Position is split into X and Y, the paste does nothing and the status line asks you to join them first.

## Checks behind it

- `b869_paste_path_into_position` (app tests): 11 of 11, `verification/B-869_paste_path_table.md`.
- A private background copy of the app, driving the real keys (M, name click, Ctrl+C, P, Ctrl+V, Ctrl+Z): 9 of 9, `verification/B-869_private_app_table.md`.
