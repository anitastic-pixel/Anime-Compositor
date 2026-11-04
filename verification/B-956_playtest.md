# B-956 playtest: Space moves a shape or mask while drawing it (D-1076)

The tutorial (4GeGLgK2BBk, about 3:00) drags out a shape, holds Space to slide it into place,
lets go of Space to carry on sizing it, then lets go of the mouse. This is a change to the window
only, so there is no picture: the check is the steps below and the table
`verification/B-956_space_moves_while_drawing_table.md`.

## Steps

1. Open the reference shot and choose a layer. Pick the Rectangle tool (Q).
2. Press the mouse on the picture and drag: a rectangle grows from where you pressed.
3. Still holding the mouse, hold Space and keep moving. The whole rectangle slides with the
   pointer and keeps its size. Playback does not start.
4. Let go of Space and keep moving: the rectangle is sized again, from its new corner.
5. Let go of the mouse. The mask is made where you left it. One Ctrl+Z removes it.
6. Do the same with the Ellipse, Polygon and Star tools, and with no layer chosen (a shape layer
   is made). Each slides with Space held.
7. With no drag going on, Space still plays and stops as before.

Wrong if:
- Space during the drag starts or stops playback;
- the shape changes size while Space is held;
- after letting go of Space the shape jumps back to where it was first pressed.

Not the same as After Effects:
- The pen tool does not take Space here.
