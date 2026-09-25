# B-32c: Selective Colour Blur's Tolerance, by hand

Built on 2026-09-25 against D-88, which you accepted the same day ("let's do tolerance").

The generated halves are `verification/B-31b_selective_blur_table.md`, 101 of 101, which now
renders FX-SELBLUR-025 to 033 as well, and `verification/B-12b_state_fields_table.md`, which
checks the card sends the tolerance the command reads. This sheet covers what the tables cannot:
how it looks on a drawing.

## Before you start

The test drawing is `verification/B-32c_tolerance_drawing.png`: B-31c's face, but with the
skin and shadow painted unevenly, as a painted or antialiased drawing is. Each of their pixels is
up to 3 steps off in red, green and blue, so the drawing has 690 colours and only 32 of its
30,000 pixels are exactly skin `#f6d6be` or shadow `#dba08e`. Lines, highlight and blush are
left exact.

Import it, make a layer from it, press **Full resolution**, and zoom in on the shadow's edge.
Add **Selective Colour Blur** and give it the two colours `f6d6be` and `dba08e`, as in B-31c.

## What to check

1. **The box.** The card has a **Tolerance, 0 to 255** box under Blur, at 0. Like Blur, drag
   it sideways or click and type a number.
2. **Tolerance 0 is exact.** At 0 the picture barely changes: almost no pixel is exactly one of
   the two colours. This is B-31c's rule.
3. **Tolerance 3.** Set it to 3. Now every skin and shadow pixel counts: the skin's grain
   smooths out and the edge between skin and shadow goes soft, as it did on B-31c's clean
   drawing.
4. **Lines still stop it.** The black lines stay sharp, and the softening still stops at them.
5. **Tolerance 21.** The blush is 21 steps from the shadow colour, so from here it counts too
   and its edge goes soft into the skin. At 20 it is still sharp.
6. **Tolerance 42.** The highlight is 42 steps from the skin colour: from 42 its edge goes soft
   as well. At 41 it is still sharp.
7. **Tolerance 189.** The lines count from here, and go soft. A large tolerance takes in the
   lines; keep it low on a real drawing.
8. **Out of range.** Type 300: it is refused with a sentence saying tolerance runs from 0 to
   255, and the card keeps its old number.
9. **Keyed.** Click the key diamond beside Tolerance at frame 0 with it at 0, go to a later
   frame and set it to 3. Scrub between: the skin goes from grainy to smooth when it reaches 1.
10. **Undo.** Press **Ctrl+Z**: the undo list says the Selective Colour Blur settings changed,
    and each step goes back one change.
11. **Saved.** Save, close and open again: the tolerance and its keys are still there.
12. **Your own drawings.** Try it on the grass painting and the purple flame. Start low, around
    5 to 20, and raise it until the colours you want are taken in.

## Known limits, on purpose

- Tolerance measures each of red, green and blue on its own, in 8-bit steps: a colour counts
  when all three are within the tolerance of one chosen colour. It is not a "looks alike" measure.
- It is not rounded: 0.5 takes in nothing that 0 does not.

## What to answer

"works", or which step number did something else and what it did.
