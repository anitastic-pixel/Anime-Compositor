# B-72: Gradient Map, by hand

Built on 2026-09-26 against D-129, which you accepted the same day as the seventh of the second
batch of ten ("go ahead with the ten").

The generated halves are `verification/B-72_gradient_map_table.md`, 90 of 90, which renders every
FX-GRADMAP case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Gradient Map card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-72a proposal/` shows what to expect.

## Before you start

Open a project with a coloured drawn layer, a character or a background with light and dark
parts, select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Gradient Map** in **Add effect…**. It goes to the end of the stack, and
   the card shows **Shadow Colour** #000000, **Midtone Colour** #808080, **Highlight Colour**
   #ffffff, **Midpoint** 50 and **Amount** 100. The drawing turns grey, light parts light and
   dark parts dark.
2. **Sunset.** Shadow #2a1650, midtone #c85a50, highlight #ffe6b4: the darks go deep violet, the
   middles warm red and the lights pale gold.
3. **Amount.** Amount 50: halfway between the drawing's own colours and the mapped ones.
   Amount 0: the drawing as it was.
4. **Midpoint.** Midpoint 25: more of the drawing takes the highlight side. Midpoint 75: more
   takes the shadow side.
5. **Reversed.** Shadow #ffffff, highlight #000000: a negative.
6. **Edges.** Soft line edges stay soft and take the line's colour; empty parts stay empty.
7. **Out of range.** Type 100 in Midpoint: it is refused with a sentence saying it runs from 1
   to 99, and the card keeps its old number.
8. **Keyed.** Key Amount at 0 and at 100 at a later frame. Play: the grade fades in.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- Three colours only, no free list of colour stops.
- A strong colour and a grey of the same brightness take the same colour, so a red and a blue of
  similar brightness map nearly alike.
- A midtone colour far off the line between the other two can show a crease at the midpoint in
  a smooth shading.
- It is modelled on After Effects' Gradient Map and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
