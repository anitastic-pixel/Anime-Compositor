# B-952 playtest: Crop comp to region of interest (D-1072)

The tutorial (4GeGLgK2BBk, about 4:40) draws a Region of Interest on the viewer, then picks
Composition > Crop Comp to Region of Interest. The composition shrinks to the box and the picture
inside the box does not move.

## Steps

1. Open a project with a few layers on screen (the reference shot will do), with none of them
   locked.
2. Click the **Region of interest** button in the viewer's bar and drag a box over part of it.
   Only the box is drawn.
3. Open the **Composition** menu and pick **Crop comp to region of interest** (the command
   finder has it too). The status line says "Cropped to W by H".
4. The composition is now the size of the box. What you see is exactly what was inside the box,
   in the same places. The dashed box is gone.
5. Play it. Layers that move still move the same way inside the new frame.
6. Open Composition settings (Ctrl+K): the width and height are the box's.
7. Press Ctrl+Z twice. The composition is its old size again and every layer is back where it was.
8. Lock one layer that has no parent, draw a box and crop again. Nothing changes, and the status
   line asks you to unlock that layer first.
9. Pick the crop with no box drawn. The status line asks you to draw one first.

## What you should see

`verification/B-952 pictures/crop.png`: on the left, the reference shot at frame 12 with the box
outlined in orange; on the right, the cropped composition at the same scale. The right-hand
picture is the inside of the orange box, pixel for pixel. The table
`verification/B-952_crop_to_region_table.md` measured the difference as 0 at frames 0, 12 and 24.

Wrong if:
- after the crop anything inside the box has moved, grown or shrunk;
- a child layer (one with a parent) ends up in a different place from before;
- two Ctrl+Z do not put it all back.

Not the same as After Effects:
- After Effects also moves locked layers; ours asks you to unlock them first.
- A composition with a camera is not cropped; the status line says so.
- The crop is two entries to undo (the layers' moves, then the size), not one.
- A Position expression that sets a fixed place (rather than adding to the Position, as wiggle
  does) is not changed, so that layer will not stay put.
