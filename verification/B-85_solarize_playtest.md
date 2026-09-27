# B-85: Solarize, by hand

Built on 2026-09-26 against D-142, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the ninth of the thirty.

The generated halves are `verification/B-85_solarize_table.md`, which renders every FX-SOLAR
case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Solarize card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-85a proposal/` shows what to expect.

## Before you start

Open a project with a coloured drawn layer that has dark lines, mid colours, light skin and
some white, with empty space around it. Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Solarize** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Threshold** 128. The dark parts stay as they were; the light parts turn dark and
   strangely coloured, the darkroom look. White turns black.
2. **Threshold 0.** Every colour turns to its opposite, as Invert does.
3. **Threshold 255.** Only pure white (and any channel already at full) turns over; everything
   else stays.
4. **Hue shifts.** Threshold 200 on light skin: the red and green turn over but the blue does
   not, so the skin turns a blue-purple. Each colour channel is decided on its own.
5. **Soft edges and empty space.** Soft edges stay soft, and the empty parts stay empty.
6. **Out of range.** Type 256 in Threshold: it is refused with a sentence saying it runs from 0
   to 255, and the card keeps its old number.
7. **Keyed.** Key Threshold at 255 and at 0 at a later frame. Play: the turning-over spreads from
   the lights down into the darks.
8. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- The choice is a hard cut, per channel, on the stored (sRGB) values: a gradient crossing the
  threshold shows a sharp band where it flips. That is the solarized look, not a fault.
- A channel exactly on the threshold is turned over.
- It is modelled on After Effects' Solarize and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
