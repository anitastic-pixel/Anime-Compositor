# B-25c: a shape layer, by hand

Built on 2026-09-21 against D-78, which the owner accepted the same day. Nothing new was decided
for this; only built. The one addition the owner asked for alongside it: over the picture, the
pen tool now shows a **pen** as the mouse pointer.

The generated halves are `verification/B-25b_shape_table.md`, 51 of 51, which checks the
pixels a shape layer draws, and `verification/B-25c_panel_table.md`, 17 of 17, which checks
what the window's buttons send and what comes back. This sheet covers what neither can judge:
whether the shapes look right on the picture and whether the tools behave the way a person
expects.

## Before you start

Open any project with a composition, or use **Save As...** first and work on the copy. Nothing
below changes the fixtures.

## What to check

1. **The pen pointer.** Choose any layer and press **G**. Move the mouse over the picture: the
   pointer is a small **pen nib**, and a point goes where its **tip** is (bottom left of the
   nib). Press **Q**: the pointer is a crosshair again. Press **V**: the ordinary arrow.
2. **A new shape layer.** Press **New shape layer** beside New solid. A layer called **Shape
   Layer 1** appears on the timeline, with a plain bar and no drawing. The picture does not
   change, because it has no shapes yet. The pen is already in hand, and its panel says
   "Shapes: none yet".
3. **A rectangle.** With the shape layer chosen, press **Q** and drag across the picture. A
   **grey rectangle** is drawn where you dragged, solid and not see-through. Its row appears on
   the timeline under the layer, reading "Shape 1, closed, 4 points".
4. **An ellipse.** Press **Q** again (it switches to the ellipse) and drag somewhere else. A grey
   ellipse, **Shape 2**, is drawn over the rectangle where they overlap.
5. **An open line with the pen.** Press **G**, click three or four places across the picture and
   press **Enter**. A **white line 4 pixels wide** runs through the points, not closed back to
   the first. Its row reads "open".
6. **A closed pen shape.** Press **G** again, click four points and then click the **first**
   point again. The shape closes and is filled grey.
7. **A shape may cross itself.** Draw a figure-of-eight with the pen and close it. Unlike a
   mask, it is drawn, not refused.
8. **Fill and stroke.** In the layer's panel, pick a red **Fill** colour for Shape 1: it turns
   red. Press **Add stroke**, pick blue and drag the stroke's **width** to about 20: a thick
   blue outline appears around it, with **rounded corners**. Drag the fill's opacity down to
   50%: the picture beneath shows through the fill but not through the stroke. Press **No
   fill**: only the outline is left.
9. **Open and closed.** Press **Closed** on the rectangle: it becomes **Open**, the side from the
   last corner back to the first disappears from its stroke, and the fill still fills. Press it
   again to close it.
10. **Points.** Press V, then drag a corner of a shape. The shape follows the hand, and one
    Ctrl+Z takes back the whole drag.
11. **Moving shapes.** Press the diamond on a shape's row at frame 0, go to frame 24 and drag a
    point. Scrub between: the shape moves between the two, as a mask's path does.
12. **Masks still work elsewhere.** Choose an ordinary drawing layer and press **Q**: dragging
    draws a **mask** on it, as before, not a shape.
13. **Saved and reopened.** Save, then **Open** the same file. Every shape, its colours, its
    stroke and its keys are as they were, and nothing is reported in the diagnostics.
14. **Export.** Export the range as PNG. The shapes are in the frames as the viewer showed them.

## Known limits

- On a shape layer the picture's tools draw **shapes**, so a shape layer's own masks can be
  set in its panel but not drawn on the picture.
- A new shape is always named "Shape N" and grey (closed) or white (open). Change its colours in
  the panel after drawing.
- The pen's points are straight corners, as they are for a mask. Pulling handles out while
  placing a point, as After Effects allows, is not built. Curves come from the ellipse tool,
  whose points' handles can be dragged.
- A shape's key, like a mask's, cannot be dragged along its row, eased with F9, or opened in the
  graph editor, and is always linear. That is the next piece of work, for masks and shapes both.
  (B-24e, 2026-09-22, has since built the dragging and the easing: `verification/B-24e_path_key_playtest.md`.)

## Result

Pass / fail, and what was seen if it failed:

