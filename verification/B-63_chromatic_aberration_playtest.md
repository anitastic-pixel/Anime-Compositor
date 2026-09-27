# B-63: Chromatic Aberration, by hand

Built on 2026-09-26 against D-120, which you accepted the same day as the tenth and last of the
batch of ten ("proceed with said batch").

The generated halves are `verification/B-63_chromatic_aberration_table.md`, 68 of 68, which
renders every FX-CHROMA case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Chromatic Aberration card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks and
feels in the window. The picture in `verification/B-63a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Chromatic Aberration** in **Add effect…**. It goes to the end of the
   stack, and the card shows **Amount** 3 and **Centre** 50, 50. Thin red and blue fringes
   appear on the drawing's outer edges, widest furthest from the middle, as in the picture's
   second panel.
2. **Stronger.** Amount 12: wide fringes, as in the third panel. Nothing near the middle moves.
3. **Off centre.** Centre 0, 0: the split runs away from the top left corner, and the far side
   of the drawing splits most, as in the fourth panel.
4. **Amount 0.** The drawing is as it was.
5. **Edges.** A fringe that would fall outside the layer's own box is cut off at it.
6. **Out of range.** Type 101 in Amount: it is refused with a sentence saying it runs from 0 to
   100, and the card keeps its old number.
7. **Keyed.** Key Amount at 0 and at 12 at a later frame. Scrub between: the colours slide apart
   smoothly.
8. **Draft.** Press **Draft**: the picture is smaller, but the fringes are about as wide
   relative to the drawing.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- The layer does not grow, so a strong split on a drawing that fills its box loses the part of
  a fringe past the edge. An effect above it that grows the layer, such as Outline or Drop
  Shadow, gives the fringe room.
- Over empty pixels a fringe is pure red or pure blue, since there is no green there to mix
  with.
- Only red and blue move; there is no per-channel angle or the barrel distortion of After
  Effects' Optics Compensation.

## What to answer

"works", or which step number did something else and what it did.
