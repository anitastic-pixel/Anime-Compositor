# B-38: Line Width, by hand

Built on 2026-09-25 against D-94, which you accepted the same day as the fourth of the batch of
seven ("knowing the expectation, let's do a batch of 1 thru 7 in your recommended order").

The generated halves are `verification/B-38_line_width_table.md`, 81 of 81, which renders every
FX-WIDTH case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, 87 of 87, which checks the Line Width card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks and feels
in the window. The picture in `verification/B-38a proposal/` shows what to expect.

## Before you start

Use the face drawing again, `verification/B-31c_selective_blur_drawing.png`. Import it, make a
layer from it and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Line Width** in **Add effect…**. It goes to the top of the stack (below
   any Selective Colour Blur), and the card shows **Width Of** set to **The whole shape**,
   **Width** 1 and **Tolerance** 0, with no colour list. The whole face is one pixel bigger all
   round.
2. **Bigger.** Set Width to 4: the outline is clearly thicker on the outside, and the edge is not
   cut off in a straight line anywhere.
3. **Smaller.** Set Width to -2: the whole face shrinks two pixels, and its outline gets thinner
   from the outside.
4. **Just the line.** Set **Width Of** to **Chosen colours**: the colour list appears. Choose
   the line #1e1a24 and set Width to 2. The outline and the mouth line get thicker, and the skin,
   highlight and blush stay where they were except where the line now covers them.
5. **Thinner line.** Set Width to -1: the outline gets thinner. The slanted mouth line goes
   dotted; that is the known limit below, not a fault.
6. **Colours kept.** Set **Width Of** back to **The whole shape**: the colour list hides, and
   the face grows or shrinks as a whole again. Set it to Chosen colours once more: the line
   colour is still chosen.
7. **Out of range.** Type 25 in Width: it is refused with a sentence saying the width runs from
   -20 to 20, and the card keeps its old number.
8. **Keyed.** Key Width at -3 and at 3 at a later frame. Scrub between: the line thins, then
   thickens, with no jumps back.
9. **Draft.** Press **Draft**: the picture is smaller, but the line is about as thick compared
   to the face as at full resolution.
10. **Undo and saved.** Ctrl+Z steps back each change, the Width Of choice included. Save, close
    and open again: every setting and key is still there.

## Known limits, on purpose

- It works in whole pixels, so thinning a thin slanted line leaves it dotted (step 5). A
  smoother thinning is not in D-94.
- A large width on a large layer is slow; every pixel looks at up to about 1,250 pixels around it.
  P-17, the bug and performance pass after the batch, measures it.

## What to answer

"works", or which step number did something else and what it did.
