# B-77: Invert, by hand

Built on 2026-09-26 against D-134, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the first of the thirty.

The generated halves are `verification/B-77_invert_table.md`, 78 of 78, which renders every
FX-INVERT case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Invert card sends every setting the
command reads. This sheet covers what the tables cannot: how it looks and feels in the window.
The picture in `verification/B-77a proposal/` shows what to expect.

## Before you start

Open a project with a coloured drawn layer, a character with a clear outline and some empty
space around it, select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Invert** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Channel** at **RGB** and **Amount** at 100. The drawing turns to a negative:
   black lines turn white, skin turns a deep blue-green. The empty parts stay empty.
2. **Half way.** Amount 50: every colour turns the same middle grey, a flat grey silhouette.
3. **One channel.** Channel **Red**: only the red is turned. Skin turns a teal, and a pure
   white part turns cyan. **Green** and **Blue** do the same for their own channel.
4. **Alpha.** Channel **Alpha**: the drawing's shape is turned inside out. Where it was drawn
   it is now empty, and the empty space around it fills with black. Soft edges stay soft.
5. **Out of range.** Type 101 in Amount: it is refused with a sentence saying it runs from 0 to
   100, and the card keeps its old number.
6. **Keyed.** Key Amount at 0 and at 100 at a later frame. Play: the drawing fades through grey
   into its negative.
7. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- The colours are turned in their stored (sRGB) values, as a paint program does, so amount 50
  turns every colour to the grey a paint program calls the middle, about #808080.
- **Alpha fills only the layer's own box.** An inverted covering fills the empty pixels of the
  drawing's own rectangle with black; it does not fill the whole frame. A character on a small
  layer gets a black box round it, not a black screen.
- It is modelled on After Effects' Invert and is not claimed to match it. After Effects offers
  more channels (HLS, YIQ); this has RGB, red, green, blue and alpha.

## What to answer

"works", or which step number did something else and what it did.
