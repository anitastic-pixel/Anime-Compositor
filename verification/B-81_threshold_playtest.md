# B-81: Threshold, by hand

Built on 2026-09-26 against D-138, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the fifth of the thirty.

The generated halves are `verification/B-81_threshold_table.md`, which renders every FX-THRESH
case against the numbers written before the code, and `verification/B-12b_state_fields_table.md`,
which checks the Threshold card sends every setting the command reads. This sheet covers what the
tables cannot: how it looks and feels in the window. The picture in `verification/B-81a proposal/`
shows what to expect.

**One case is in dispute.** FX-THRESH-019 writes the level as a word ("128") and expects the
file to open with the effect skipped. Every other effect refuses such a file as not matching
the project format, and this build does the same. D-164 in document 14 proposes keeping it that
way. It asks you to choose; the table shows the case as "in dispute", not as passed.

## Before you start

Open a project with a coloured drawn layer, a cel with lit skin and shadow skin, and some empty
space around it, select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Threshold** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Level** at 128. The drawing turns pure black and white: dark lines and dark
   colours black, light ones white. Soft edges stay soft, and the empty parts stay empty.
2. **Low.** Level 20: almost everything turns white; only the darkest parts stay black.
3. **High.** Level 250: almost everything turns black; only pure white stays white.
4. **Splitting the cel.** Find the level where the shadow skin turns black and the lit skin stays
   white, around 172 for a usual skin. That is the cel's light and shadow as a matte.
5. **Red against blue.** A pure red counts as lighter than a pure blue: at level 50 red is white
   while blue is black.
6. **Out of range.** Type 256 in Level: it is refused with a sentence saying it runs from 0 to
   255, and the card keeps its old number.
7. **Keyed.** Key Level at 0 and at 255 at a later frame. Play: black spreads from the dark parts
   to the light ones until only white is left.
8. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- Lightness is worked on the stored (sRGB) values with the usual weights (green counts most, blue
  least), so a pixel exactly on the level turns white.
- There is no softness: the change is a hard cut, as in a paint program.
- It is modelled on After Effects' Threshold and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did. And for D-164: "keep
refusing", or "open it and skip the effect".
