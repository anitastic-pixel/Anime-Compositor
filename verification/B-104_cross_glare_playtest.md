# B-104: Cross Glare, by hand

Built on 2026-09-26 against D-161, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twenty-eighth of the thirty.

The generated halves are `verification/B-104_cross_glare_table.md`, which renders every
FX-GLARE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Cross Glare card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-104a proposal/` shows what to expect.

## Before you start

Open a project with a dark background and, above it, a drawing with a few small bright points:
a highlight in an eye, a glint on a blade, a star. Select the drawing and press
**Full resolution**.

## What to check

1. **Adding it.** Pick **Cross Glare** in **Add effect…** on the drawing. It goes to the end of
   the stack, and the card shows **Threshold** 80, **Length** 40, **Points** 4, **Angle** 45,
   **Intensity** 1 and **Colour** white. Each bright point sprouts a white X-shaped glint whose
   arms fade out about 40 pixels away. Dark parts get none.
2. **Upright cross.** Angle 0: the glints turn into a + shape.
3. **Star.** Points 6: six-armed stars; Points 8: eight arms. Points 1: a single streak.
4. **Length.** Length 100: longer arms, reaching past the drawing's edge without being cut off.
5. **Threshold.** Threshold 95: only the very brightest points glint. Threshold 50: many more
   parts glint, and large bright areas smear into broad crosses.
6. **Intensity and colour.** Intensity 3: brighter arms. Colour a warm yellow: yellow glints.
7. **Out of range.** Type 9 in Points: it is refused with a sentence saying it runs from 1 to
   8, and the card keeps its old number.
8. **Keyed.** Key Length at 0 and at 60 a second later. Play: the glints grow out from the
   points, the cartoon "ting!".
9. **Draft preview.** Switch to a half-size draft: the arms reach the same distance on screen
   as at full size.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- **Straight, even arms** with no lens rainbow colours, no spin over time and no per-arm
  lengths.
- **Every bright pixel glints**, so a large bright area gives a smeared cross the size of the
  area. Raise the threshold to keep glints to the brightest points.
- A black colour lays a dark cross rather than nothing, because the arms add covering.
- **Slow when long.** A long eight-armed glare on a large drawing takes noticeably longer.
- It is modelled on the anime cross filter and star-glint effects in spirit, not on any one
  After Effects effect.

## What to answer

"works", or which step number did something else and what it did.
