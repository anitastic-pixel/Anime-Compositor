# B-37: Select Colour, by hand

Built on 2026-09-25 against D-93, which you accepted the same day as the third of the batch of
seven ("knowing the expectation, let's do a batch of 1 thru 7 in your recommended order").

The generated halves are `verification/B-37_select_color_table.md`, 70 of 70, which renders
every FX-SELECT case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, 85 of 85, which checks the Select Colour card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks and feels
in the window. The picture in `verification/B-37a proposal/` shows what to expect.

## Before you start

Use the face drawing again, `verification/B-31c_selective_blur_drawing.png`. Import it, make a
layer from it and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Select Colour** in **Add effect…**. It goes to the top of the stack
   (below any Selective Colour Blur), and the card shows **Keep** set to **Chosen colours**,
   **Tolerance** 0 and no colour chosen. The face is untouched.
2. **The line alone.** Choose the line colour #1e1a24. Only the face's outline and the mouth
   line are left; everything else is see-through.
3. **Everything else.** Set **Keep** to **Everything else**: the lines vanish and the rest of
   the face comes back, with gaps where the lines were.
4. **Two colours.** Keep **Chosen colours** and add the skin #f6d6be and the shadow #dba08e
   instead of the line. The skin and shadow are left; the lines, the highlight and the blush
   are gone.
5. **Tolerance.** Choose #1e1a24 only and raise Tolerance slowly: nothing changes until the
   line's near colours are taken in, and nothing jumps back and forth.
6. **Out of range.** Type 300 in Tolerance: it is refused with a sentence saying the tolerance
   runs from 0 to 255, and the card keeps its old number.
7. **Keyed.** Key Tolerance at 0 and at 60 at a later frame. Scrub between: the kept area
   changes as you scrub.
8. **Undo.** Press **Ctrl+Z**: each step goes back one change, the Keep choice included.
9. **Saved.** Save, close and open again: the colours, the Keep choice and the tolerance are
   still there.

## Known limits, on purpose

- A pixel is either kept or cleared; there is no soft edge to the choice. Colour Key (D-97,
  later in the batch) has a softness.
- With colours chosen that match nothing and Keep on Chosen colours, the layer is empty. That
  is the rule, not a fault.

## What to answer

"works", or which step number did something else and what it did.
