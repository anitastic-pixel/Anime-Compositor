# B-97: Motion Tile, by hand

Built on 2026-09-26 against D-154, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twenty-first of the thirty.

The generated halves are `verification/B-97_motion_tile_table.md`, which renders every FX-TILE
case against the numbers written before the code, and `verification/B-12b_state_fields_table.md`,
which checks the Motion Tile card sends every setting the command reads. This sheet covers what
the tables cannot: how it looks and feels in the window. The picture in
`verification/B-97a proposal/` shows what to expect.

## Before you start

Open a project with a small drawing, a character or a prop about a quarter of the frame across,
in the middle of the frame. Select its layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Motion Tile** in **Add effect…**. It goes to the end of the stack, and
   the card shows **Output width** 100, **Output height** 100 and **Mirror** Off. The picture
   is unchanged: there is only one tile, the drawing itself.
2. **Wider.** Output width 300: copies of the drawing appear to its left and right, touching it
   edge to edge.
3. **Both ways.** Output height 300 as well: a grid of copies, three across and three down.
4. **Mirrored.** Mirror On: the copies beside the drawing are turned over left to right and the
   ones above and below turned upside down, so neighbouring tiles meet without a hard seam.
5. **Moving.** Move the layer: the whole grid moves with it.
6. **Out of range.** Type 99 in Output width: it is refused with a sentence saying it runs from
   100 to 1000, and the card keeps its old number.
7. **Keyed.** Key Output width at 100 and at 1000 at a later frame. Play: copies spread out to
   the sides, filling the frame.
8. **Draft preview.** Switch to a half-size draft: the same grid as at full size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- **The copies are of the layer's whole box**, empty parts included, so a drawing with empty
  space round it tiles with gaps.
- The growth rounds up to whole pixels, so an odd size can make the grid a pixel wider than the
  per cent says.
- There is no offset setting for the tiles; to slide a repeating background, put an Offset
  before it or move the layer.
- On a layer that already fills the frame, the copies land outside the frame and are not seen.
- It is modelled on After Effects' Motion Tile and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
