# B-39: Radial Blur, by hand

Built on 2026-09-25 against D-95, which you accepted the same day as the fifth of the batch of
seven ("knowing the expectation, let's do a batch of 1 thru 7 in your recommended order").

The generated halves are `verification/B-39_radial_blur_table.md`, 76 of 76, which renders every
FX-RADIAL case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, 89 of 89, which checks the Radial Blur card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks and feels
in the window. The picture in `verification/B-39a proposal/` shows what to expect.

## Before you start

Use the face drawing again, `verification/B-31c_selective_blur_drawing.png`. Import it, make a
layer from it and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Radial Blur** in **Add effect…**. It goes to the bottom of the stack,
   and the card shows **Type** set to **Spin**, **Amount** 10 and **Centre** 50 and 50. The mouth
   line smears round the middle of the face; the round outline hardly changes, because it runs
   round the centre too.
2. **More.** Set Amount to 40: the mouth line and the highlight smear much further round, the
   parts furthest from the middle most.
3. **Zoom.** Set **Type** to **Zoom** and Amount to 20: now everything smears outward and inward
   along lines from the middle, and the outline goes soft all round, as in the picture.
4. **Moving the centre.** Set Centre to 20 and 20: the smears now run round, or out from, a
   point up and to the left of the middle. Drag either Centre number: the picture follows.
5. **Off the drawing.** Set Centre to -200 and 50: the centre is well off the left of the face,
   so a spin smears the face almost straight up and down.
6. **Out of range.** Type 150 in Amount: it is refused with a sentence saying the amount runs
   from 0 to 100, and the card keeps its old number. Type 2000 in either Centre number: refused
   the same way, saying -1000 to 1000.
7. **Keyed.** Key Amount at 0 and at 40 at a later frame. Scrub between: the smear grows
   smoothly, with no jumps. Key Centre at two places too: the centre travels between them.
8. **After another blur.** Add a **Directional Blur** and move it above the Radial Blur. The
   radial smear still turns about the same point on the face.
9. **Draft.** Press **Draft**: the picture is smaller, but the smear looks the same shape.
10. **Undo and saved.** Ctrl+Z steps back each change, the Type included. Save, close and open
    again: every setting and key is still there.

## Known limits, on purpose

- It takes about one sample a pixel along each smear, so a thin line crossing a smear shows fine
  stripes, as the picture's spin panels do. Smoother is not in D-95.
- The layer does not grow: a smear that would reach past the drawing's edge is cut off there.
- A large amount on a large layer is slow: a pixel far from the centre takes up to 256 samples.
  P-17, the bug and performance pass after the batch, measures it.

## What to answer

"works", or which step number did something else and what it did.
