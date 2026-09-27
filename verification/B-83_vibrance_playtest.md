# B-83: Vibrance, by hand

Built on 2026-09-26 against D-140, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the seventh of the thirty.

The generated halves are `verification/B-83_vibrance_table.md`, which renders every
FX-VIBRANCE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Vibrance card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-83a proposal/` shows what to expect.

## Before you start

Open a project with a coloured drawn layer that has both dull colours (skin, a tan background,
a greyish shadow) and vivid ones (a saturated red or blue), with some empty space around it.
Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Vibrance** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Vibrance** 0 and **Saturation** 0. The drawing does not change at all.
2. **Vibrance up.** Vibrance 100: the dull colours (skin, tan) get clearly richer, while the
   already vivid red or blue hardly move. Greys, black and white stay exactly as they were.
3. **Saturation up.** Vibrance 0, Saturation 100: every colour gets richer by the same amount,
   so the vivid ones now clip and flatten, which is what Vibrance avoids.
4. **All grey.** Saturation -100: the drawing turns grey, keeping how bright each colour was.
5. **Faded.** Vibrance -100, Saturation 0: the dull colours go almost grey while the vivid ones
   keep most of their colour.
6. **Soft edges and empty space.** Soft edges stay soft, and the empty parts stay empty.
7. **Out of range.** Type 101 in Vibrance: it is refused with a sentence saying it runs from
   -100 to 100, and the card keeps its old number. The same for Saturation.
8. **Keyed.** Key Saturation at -100 and at 100 at a later frame. Play: the drawing goes from
   grey, through its own colours, to over-rich.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- There is **no skin-tone protection**. After Effects' Vibrance is said to spare skin; this one
  treats skin like any other dull colour, so at high Vibrance faces get richer too.
- "Dull" is measured on the stored (sRGB) values: how far apart a colour's strongest and
  weakest channels are. Grey is the colour's brightness with the usual weights (green counts
  most, blue least).
- It is modelled on After Effects' Vibrance and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did. And whether faces going
richer at high Vibrance is acceptable, or skin should be spared.
