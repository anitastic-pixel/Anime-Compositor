# B-84: Leave Color, by hand

Built on 2026-09-26 against D-141, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the eighth of the thirty.

The generated halves are `verification/B-84_leave_color_table.md`, which renders every
FX-LEAVE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Leave Color card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-84a proposal/` shows what to expect.

## Before you start

Open a project with a colourful drawn layer (a character with a red ribbon, skin, blue clothes
and green leaves, say) and some empty space around it. Select the layer and press
**Full resolution**.

## What to check

1. **Adding it.** Pick **Leave Color** in **Add effect…**. It goes to the end of the stack. The
   card shows **Colour to keep** red (#ff0000), **Tolerance** 15, **Softness** 10 and
   **Amount** 100. Everything turns grey except the reds and the colours near red; skin, being
   close to red-orange, keeps most of its colour.
2. **Another colour.** Pick a blue in Colour to keep: now the blue clothes stay and the ribbon
   turns grey.
3. **Light and dark alike.** A dark red and a light pink are kept as a red is: only the hue
   counts, not how light or strong it is.
4. **Tolerance.** Raise Tolerance to 40: the warm half of the picture keeps its colour. At 100
   nothing turns grey at all.
5. **Softness.** Softness 0 gives a hard cut: a colour is either fully kept or fully grey.
   Softness 30 fades the colours near the edge of the kept range gently.
6. **Amount.** Amount 50: the other colours go only half way to grey.
7. **A grey colour.** Pick a grey in Colour to keep: nothing is kept, and the whole drawing turns
   grey.
8. **Soft edges and empty space.** Soft edges stay soft, and the empty parts stay empty.
9. **Out of range.** Type 101 in Tolerance: it is refused with a sentence saying it runs from 0
   to 100, and the card keeps its old number. The same for Softness and Amount.
10. **Keyed.** Key Amount at 0 and at 100 at a later frame. Play: the colour drains from the
    picture, all but the kept colour, the flashback look.
11. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every
    setting and key is still there.

## Known limits, on purpose

- Colours are compared by hue alone, on the stored (sRGB) values. A near-grey that happens to
  lean red is kept like a strong red, so faint tints in greys and whites can show.
- **At Softness 0 the tolerance is a cliff:** a colour just inside it is fully kept and one just
  outside is fully grey. Keying Tolerance with Softness 0 makes colours pop in and out.
- The grey a colour turns is its own brightness, as a black-and-white print.
- The colour itself cannot be keyed; only the three numbers can.
- It is modelled on After Effects' Leave Color and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
