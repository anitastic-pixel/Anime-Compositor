# B-87: Mosaic, by hand

Built on 2026-09-26 against D-144, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the eleventh of the thirty.

The generated halves are `verification/B-87_mosaic_table.md`, which renders every FX-MOSAIC
case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Mosaic card sends every setting the
command reads. This sheet covers what the tables cannot: how it looks and feels in the window.
The picture in `verification/B-87a proposal/` shows what to expect.

## Before you start

Open a project with a drawn character layer, a face with clear features, and some empty space
around it. Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Mosaic** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Size** 10. The drawing turns into square blocks ten pixels wide, each one flat
   colour: the censored-face look.
2. **Bigger and smaller.** Size 40: a few large blocks. Size 2: fine blocks, the picture nearly
   readable. Size 1: the drawing untouched.
3. **Edges.** Blocks at the drawing's edge that are partly empty come out partly see-through,
   not solid.
4. **The blocks move with the layer.** Animate the layer's position: the blocks travel with the
   drawing rather than staying fixed on the screen.
5. **Right and bottom.** If the size does not divide the drawing's width, the last column of
   blocks at the right (and the last row at the bottom) is narrower.
6. **Out of range.** Type 0 in Size: it is refused with a sentence saying it runs from 1 to
   1000, and the card keeps its old number.
7. **Keyed.** Key Size at 1 and at 40 at a later frame. Play: the picture breaks up into ever
   larger blocks, the old "pixelate out" transition.
8. **Draft preview.** Switch to a half-size draft: the blocks cover the same part of the
   picture as at full size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- **The blocks start at the drawing's top left corner**, not its middle, so a size that does not
  divide the drawing leaves a narrow row and column of blocks at its right and bottom edges.
- Square blocks only, with no separate counts across and down as After Effects offers.
- Each block is its average, worked in linear light, so a block mixing black and white comes out
  a little lighter than a paint program's average would.
- The blocks follow the drawing, not the screen: a layer rotated by its transform shows rotated
  blocks.
- A keyed size makes the block edges jump from one pixel to the next as it grows; there is no
  smooth sliding.
- It is modelled on After Effects' Mosaic and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did. And whether the blocks
should start from the drawing's middle instead of its corner.
