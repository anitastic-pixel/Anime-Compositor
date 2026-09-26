# B-61: Outline, by hand

Built on 2026-09-26 against D-118, which you accepted the same day as the eighth of the batch
of ten ("proceed with said batch").

The generated halves are `verification/B-61_outline_table.md`, 81 of 81, which renders every
FX-OUTLINE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Outline card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-61a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**. A light grey solid behind the layer makes a white band
easy to see.

## What to check

1. **Adding it.** Pick **Outline** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Outline Colour** white, **Width** 3, **Softness** 0 and **Opacity** 100. A crisp
   white band appears round the drawing, behind it, as in the picture's second panel. The
   drawing itself is unchanged.
2. **Wider, coloured.** Outline Colour #ffd23c, Width 7: a thick yellow band, as in the third
   panel, with rounded corners.
3. **Soft.** Outline Colour #ff9a3c, Width 4, Softness 9, Opacity 70: a soft orange halo, as
   in the fourth panel.
4. **Width 0.** The drawing is as it was, whatever the softness.
5. **Nothing cut off.** A wide band spreads past the drawing's own box and is not clipped at
   it.
6. **Out of range.** Type 101 in Width: it is refused with a sentence saying it runs from 0 to
   100, and the card keeps its old number.
7. **Keyed.** Key Width at 0 and at 6 at a later frame. Scrub between: the band grows smoothly
   from nothing.
8. **Draft.** Press **Draft**: the picture is smaller, but the band is about as thick relative
   to the drawing.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- The band goes behind the drawing only; there is no inside or centred stroke.
- It follows the outline of the whole layer, so a hole in the drawing is filled by the band as
  far as the width reaches, and two parts close together are joined.
- The band is counted on whole-pixel steps, so a width that is not a whole number changes the
  band in small jumps rather than smoothly; soften it a little to hide this.
- Large widths on full-size frames are slower; it runs on the processor only.

## What to answer

"works", or which step number did something else and what it did.
