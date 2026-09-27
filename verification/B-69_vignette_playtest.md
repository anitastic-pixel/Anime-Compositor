# B-69: Vignette, by hand

Built on 2026-09-26 against D-126, which you accepted the same day as the fourth of the second
batch of ten ("go ahead with the ten").

The generated halves are `verification/B-69_vignette_table.md`, 95 of 95, which renders every
FX-VIGNETTE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Vignette card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-69a proposal/` shows what to expect.

## Before you start

Open a project with a layer that covers the whole frame, a background card is ideal, select the
layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Vignette** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Amount** 50, **Size** 100, **Roundness** 0, **Softness** 50, **Centre** 50, 50
   and **Colour** #000000. The corners darken gently; the middle is untouched.
2. **Stronger.** Amount 100: the corners go fully black.
3. **Size.** Size 50: the dark ring moves in, well inside the corners. Size 200: only a faint
   darkening is left at the very corners.
4. **Softness.** Softness 0: a hard-edged oval. Softness 100: the darkening starts right at the
   centre and grows smoothly outward.
5. **Roundness.** Roundness 100: the oval becomes a circle, darker at the sides of a wide frame
   and lighter at the top and bottom.
6. **Colour.** Colour #6450a0: the edges tint violet instead of darkening.
7. **Off centre.** Centre 20, 30: the bright middle moves to the top left.
8. **Edges.** Empty parts of the layer stay empty; the vignette never spills past the drawing.
9. **Out of range.** Type 201 in Size: it is refused with a sentence saying it runs from 1 to
   200, and the card keeps its old number.
10. **Keyed.** Key Size at 200 and at 50 at a later frame. Scrub between: the ring closes in
    smoothly.
11. **Draft.** Press **Draft**: the picture is smaller, but the vignette sits in the same place
    relative to the drawing.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- It vignettes the layer it is on. For a whole shot, put it on an adjustment layer or a
  full-frame layer.
- The colour is laid on as a plain mix; there is no multiply, screen or add.
- Roundness 100 is a circle of the oval's area, so on a wide drawing it darkens the sides more.
- It is modelled on After Effects' Lumetri vignette and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
