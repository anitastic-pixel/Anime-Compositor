# B-56: Hue/Saturation, by hand

Built on 2026-09-26 against D-113, which you accepted the same day as the third of the batch of
ten ("proceed with said batch").

The generated halves are `verification/B-56_hue_saturation_table.md`, 85 of 85, which renders
every FX-HUESAT case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Hue/Saturation card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-56a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Hue/Saturation** in **Add effect…**. It goes to the end of the stack,
   and the card shows **Hue** 0, **Saturation** 0 and **Lightness** 0. Nothing changes yet.
2. **Hue.** Hue 120: skin turns green and a blue band turns pink-red, as in the picture's second
   panel. Black lines and greys stay as they are.
3. **Saturation.** Hue back to 0, Saturation -100: every colour turns grey at its own
   lightness, as in the third panel.
4. **Lightness.** Saturation back to 0, Lightness 100: everything that shows turns white;
   Lightness -100: black. Soft edges stay soft and transparent parts stay transparent.
5. **Together.** Hue -150, Saturation -30, Lightness -40: compare with the fourth panel.
6. **Out of range.** Type 200 in Hue: it is refused with a sentence saying it runs from -180 to
   180, and the card keeps its old number. Type 101 in Lightness: refused, -100 to 100.
7. **Keyed.** Key Hue at -180 and at 180 at a later frame. Scrub between: the colours run all
   the way round the colour wheel smoothly, and the first and last frames look the same.
8. **Draft.** Press **Draft**: the picture is smaller, but the colours are the same.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- It changes every colour at once; there is no choosing a range of hues to change, as After
  Effects' Channel Range has. Select Colour or Colour Key can pick colours out first.
- There is no Colorize switch; Tint does that job.
- Lightness 100 is plain white and -100 plain black, whatever the colour was.

## What to answer

"works", or which step number did something else and what it did.
