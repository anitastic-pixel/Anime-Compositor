# B-35: Line Recolour, by hand

Built on 2026-09-25 against D-91, which you accepted the same day as the first of the batch of
seven ("knowing the expectation, let's do a batch of 1 thru 7 in your recommended order").

The generated halves are `verification/B-35_line_recolor_table.md`, 75 of 75, which renders
every FX-RECOLOR case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Line Recolour card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-35a proposal/` shows what to expect.

## Before you start

The test drawing is `verification/B-31c_selective_blur_drawing.png`, the face you used for
Selective Colour Blur. It has exactly five colours, drawn without antialiasing:

| Colour | Written |
| --- | --- |
| skin | `#f6d6be` |
| shadow | `#dba08e` |
| highlight | `#fff3e8` |
| blush | `#f09696` |
| line | `#1e1a24` |

Import it, make a layer from it and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Line Recolour** in **Add effect…**. It goes to the top of the stack
   (below any Selective Colour Blur), and the card shows **Tolerance** 0, **New Colour** red
   `#ff0000` with its swatch, and **Colours, up to 8** saying no colour is chosen, so nothing
   changes. The face is unchanged.
2. **The line.** Type `1e1a24` in the Add box and press Enter: the outline and the line across
   the shadow turn red, all of them, and nothing else changes.
3. **New colour.** Type `6b3a1e` in the New Colour box and press Enter: the line is brown. Drag
   over the New Colour swatch's picker: the line follows as you move.
4. **Two colours.** Add `f09696`: the blush turns brown too.
   Remove it with its ✕: the blush is pink again.
5. **Tolerance.** Remove the line colour and add `28242e`, which is 10 away from the line on
   every channel: nothing changes. Set Tolerance to 9: still nothing. At 10: the line is brown.
6. **Out of range.** Type 300 in Tolerance: it is refused with a sentence saying the tolerance
   runs from 0 to 255, and the card keeps its old number. Type `red` in the New Colour box: it is
   refused with a sentence saying a colour is written # and six hexadecimal digits.
7. **Keyed.** Click the key diamond beside Tolerance at frame 0 with it at 0, go to a later
   frame and set it to 20. Scrub between: the line is black until the tolerance reaches 10, then
   brown.
8. **Undo.** Press **Ctrl+Z**: the undo list says the Line Recolour settings changed, and each
   step goes back one change.
9. **Saved.** Save, close and open again: the colours, the new colour, the tolerance and its
   keys are still there.
10. **Your own drawings.** Try it on a drawing of yours with a black line. Pick the line's
    colour, then raise the tolerance until the whole line changes, antialiased edge included,
    and stop before the fill starts to change.

## Known limits, on purpose

- Every chosen pixel becomes one flat colour at its own covering, so a line's own shading is
  lost; D-91 leaves a colour per chosen colour for later.
- An edge a drawing program antialiased by mixing the line into the fill, rather than by making
  it see-through, holds in-between colours; the tolerance decides how many of them change, and
  the ones it misses keep their dark tint.

## What to answer

"works", or which step number did something else and what it did.
