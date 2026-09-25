# B-36: Directional Blur, by hand

Built on 2026-09-25 against D-92, which you accepted the same day as the second of the batch of
seven ("knowing the expectation, let's do a batch of 1 thru 7 in your recommended order").

The generated halves are `verification/B-36_directional_blur_table.md`, 66 of 66, which renders
every FX-DIRBLUR case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Directional Blur card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-36a proposal/` shows what to expect.

## Before you start

Use the face drawing again, `verification/B-31c_selective_blur_drawing.png`. Import it, make a
layer from it and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Directional Blur** in **Add effect…**. It goes to the bottom of the
   stack, and the card shows **Direction** 0 and **Length** 10. The face is smeared up and down,
   about five pixels each way; its left and right edges stay sharp.
2. **Across.** Set Direction to 90: the smear turns to left and right, and the top and bottom
   edges are sharp. Set it to 270: exactly the same picture, because the streak runs both ways.
3. **Diagonal.** Set Direction to 45: the smear runs from bottom left to top right.
4. **Length.** Set Length to 0: the face is untouched. Drag Length up slowly to 30: the smear
   grows smoothly, with no jumps.
5. **Past the edge.** Set Length to 100: the smear reaches well past the rectangle of the
   drawing and is not cut off in a straight line.
6. **Out of range.** Type 600 in Length: it is refused with a sentence saying the length runs
   from 0 to 500, and the card keeps its old number. Type 4000 in Direction: refused the same
   way, from -3600 to 3600.
7. **Keyed.** Click the key diamond beside Length at frame 0 with it at 0, go to a later frame
   and set it to 40. Scrub between: the smear grows from nothing. Key Direction from 0 to 90 the
   same way: the smear turns as you scrub.
8. **Draft.** Press **Draft**: the picture is smaller but the smear is the same size compared to
   the face as at full resolution.
9. **Undo.** Press **Ctrl+Z**: the undo list says the Directional Blur settings changed, and each
   step goes back one change.
10. **Saved.** Save, close and open again: the direction, the length and their keys are still
    there.

## Known limits, on purpose

- The streak always runs equally both ways; a one-sided streak is not in D-92.
- A very long streak on a large layer is slow: every pixel takes one sample per pixel of
  length. P-17, the bug and performance pass after the batch, measures it.

## What to answer

"works", or which step number did something else and what it did.
