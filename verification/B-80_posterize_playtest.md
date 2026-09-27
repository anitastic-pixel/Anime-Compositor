# B-80: Posterize, by hand

Built on 2026-09-26 against D-137, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the fourth of the thirty.

The generated halves are `verification/B-80_posterize_table.md`, which renders every FX-POSTER
case against the numbers written before the code, and `verification/B-12b_state_fields_table.md`,
which checks the Posterize card sends every setting the command reads. This sheet covers what the
tables cannot: how it looks and feels in the window. The picture in `verification/B-80a proposal/`
shows what to expect.

## Before you start

Open a project with a drawn layer that has soft shading or a gradient in it, and some empty
space around it, select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Posterize** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Levels** at 6. Smooth shading breaks into flat bands, like a screen print. The
   empty parts stay empty.
2. **Fewest.** Levels 2: every colour snaps to one of eight hard colours (black, white, red,
   green, blue, yellow, cyan, magenta).
3. **Many.** Levels 16: the bands get close together and the drawing looks almost normal. Levels
   256: exactly the drawing.
4. **Whole numbers only.** Levels 6.9 looks exactly like 6.
5. **Out of range.** Type 1 or 257 in Levels: it is refused with a sentence saying it runs from 2
   to 256, and the card keeps its old number.
6. **Keyed.** Key Levels at 2 and at 10 at a later frame. Play: the bands step finer in jumps,
   not smoothly, because only whole levels count.
7. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- The steps are cut in the stored (sRGB) values, as a paint program does, so the steps look even
  to the eye.
- **Levels 255 is not "nearly untouched".** 255 steps is one fewer than a stored colour has, so
  every in-between colour is nudged by up to one level. Only 256 leaves the drawing exactly.
- Each channel is stepped on its own, so a pale colour can pick up a slight tint at low levels.
- It is modelled on After Effects' Posterize and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
