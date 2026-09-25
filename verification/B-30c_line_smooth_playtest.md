# B-30c: Line Smoothing in the Effects panel, by hand

Built on 2026-09-25 against D-86, which you accepted the same day ("proceed with B").

The generated halves are `verification/B-30b_line_smooth_table.md`, 54 of 54, which renders
every FX-SMOOTH case from document 25 and compares it with the reference numbers, and two panel
tables: `verification/B-12a_effects_table.md`, which adds Line Smoothing from the window and
checks it lands at the top of the stack at softness 50 and threshold 10, and
`verification/B-12b_state_fields_table.md`, which checks the two boxes the panel sends are the
settings the command reads. This sheet covers what the tables cannot: how it looks on a
drawing.

## Before you start

Open any project you are happy to change without saving. The test drawing is
`verification/B-30c_line_smooth_drawing.png`, 320 by 200: a head with a black outline, a
shadow with a slanted edge, a thin red line, and a blue box with square corners, all drawn
without antialiasing. Click **Import drawings...**, choose it, and make a layer from it. Press
**Full resolution** if it is not already on, and zoom the viewer in (Ctrl with the mouse wheel,
or the Zoom slider) until you can see the stair steps on the outline and the red line.

## What to check

1. **In the list.** With the layer selected, open **Add effect...** under Effects. **Line
   Smoothing** is the last choice, after Tint.
2. **Adding it.** Choose it. A card named Line Smoothing appears with two boxes, **Softness,
   0 to 100** at 50 and **Threshold, 0 to 255** at 10. In the viewer the steps on the outline,
   the shadow's edge and the red line turn into smooth slopes.
3. **Corners kept sharp.** The blue box's four corners stay square. Its straight edges do not
   change at all.
4. **No fringe.** Turn the checkerboard on (**Hide grid** off). Round the outside of the head,
   where the outline meets nothing, the smoothed pixels are dark and partly see-through. There
   is no white or grey ring.
5. **Top of the stack.** Add a **Blur** to the same layer, then add a second **Line
   Smoothing**. The new one goes to the top of the list of cards, above the blur, not below
   it. Delete that second one.
6. **Softness.** Drag the Softness box down to 0: the steps come back exactly as drawn. Up to
   100: the slopes are longer and softer than at 50.
7. **Threshold.** Set Threshold to 255. Every colour now counts as one, so nothing is smoothed.
   Set it back to 10.
8. **Out of range.** Type 150 in Softness. It is refused with a sentence saying softness runs
   from 0 to 100, and the card keeps its old number.
9. **Keyed.** Click the key diamond beside Softness on frame 0 with Softness at 0. Go to a
   later frame and set it to 100. Scrub between: the smoothing grows from nothing.
10. **Bypass and undo.** Switch the effect off on its card: the steps come back. Switch it on,
    then press **Ctrl+Z** a few times: each step goes back one change.
11. **Saved.** Save the project (**Ctrl+S**), close the window and open it again. The card is
    there with its settings and keys, and the picture is the same.
12. **Your own drawing.** If you have a drawing made without antialiasing, import it and try
    Line Smoothing on it too.

## Known limits, on purpose

- A very shallow line, whose steps are long, becomes a long soft fade along each step.
- A corner shorter than 4 pixels on either side is still rounded (FX-SMOOTH-006).
- It smooths every colour boundary on the layer. Choosing which line colours are smoothed, and
  making lines thicker or thinner, are not in D-86.

## What to answer

"works", or which step number did something else and what it did.
