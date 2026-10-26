# B-766 playtest: only the changed part of the picture is sent (D-886)

This matters when the viewer is drawn by the processor. Set the viewer to the CPU first
(the graphics card / CPU setting above the viewer).

1. Open a project with many layers (the 25-layer reference shot is a good one).
2. Set the viewer to Draft. Drag a small layer around for a few seconds.
3. Drag the playhead back and forth across the timeline.
4. Drag one mask point.
5. Set the viewer to Full and do steps 2 to 4 again.

What should feel different: on the 25-layer shot, moving a layer and dragging the playhead keep
up more closely with your hand, in Draft and in Full. On a small project you may feel no
difference; that is expected.

What must not happen (any of these is a fail):
- a patch or rectangle of an older picture left behind anywhere in the viewer;
- the picture not matching the layer's final place after you let go;
- any flicker or blank area while dragging.

If in doubt, press Full or move one frame forward and back: the picture must look exactly the
same as before you did.
