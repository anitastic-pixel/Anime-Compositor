# B-79: Black & White, by hand

Built on 2026-09-26 against D-136, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the third of the thirty.

The generated halves are `verification/B-79_black_white_table.md`, which renders every FX-BW case
against the numbers written before the code, and `verification/B-12b_state_fields_table.md`,
which checks the Black & White card sends every setting the command reads. This sheet covers what
the tables cannot: how it looks and feels in the window. The picture in
`verification/B-79a proposal/` shows what to expect.

## Before you start

Open a project with a colourful drawn layer (red, yellow, green, blue and skin all showing) and
some empty space around it, select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Black & White** in **Add effect…**. It goes to the end of the stack, and
   the card shows six numbers: **Reds** 40, **Yellows** 60, **Greens** 40, **Cyans** 60,
   **Blues** 20, **Magentas** 80. The drawing turns grey. The empty parts stay empty.
2. **One range.** Move **Reds** up to 200: red things and the skin turn much lighter grey, and the
   blue and green things do not change. Move it down to -100: they turn dark.
3. **A red filter.** Reds 120, Yellows 110, Greens -10, Cyans -50, Blues -50, Magentas 120: warm
   colours glow pale, blue sky and greens go dark, like a black and white photo through red glass.
4. **All the same.** All six at 100: every pure colour turns white. All six at 0: every pure
   colour turns black.
5. **Out of range.** Type 301 in any of the six: it is refused with a sentence saying it runs
   from -200 to 300, and the card keeps its old number.
6. **Keyed.** Key **Blues** at -200 and at 300 at a later frame. Play: the blue parts brighten
   from black to white, the rest stays still.
7. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- It is worked on the stored (sRGB) values, as a paint program does. Greys, whites and blacks
  stay as they are whatever the six numbers.
- There is no tint setting. For a sepia look, put **Tint** or **Gradient Map** after it.
- It is modelled on the Black & White adjustment of paint programs (After Effects has the same
  one) and is not claimed to match either.

## What to answer

"works", or which step number did something else and what it did.
