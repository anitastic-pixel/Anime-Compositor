# B-55: Levels, by hand

Built on 2026-09-26 against D-112, which you accepted the same day as the second of the batch of
ten ("proceed with said batch").

The generated halves are `verification/B-55_levels_table.md`, 88 of 88, which renders every
FX-LEVELS case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Levels card sends every setting the
command reads. This sheet covers what the tables cannot: how it looks and feels in the window.
The picture in `verification/B-55a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Levels** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Input Black** 0, **Input White** 255, **Gamma** 1, **Output Black** 0 and
   **Output White** 255. Nothing changes yet.
2. **Input.** Input Black 40 and Input White 215: the darks go black and the lights go white,
   the colours stronger, as in the picture's second panel.
3. **Gamma.** Back to 0 and 255, and Gamma 1.8: the middle tones lighten and the line turns
   grey-brown, as in the third panel. Black and white stay.
4. **Output.** Gamma 0.8, Output Black 70 and Output White 200: everything flattens towards a
   middle grey, as in the fourth panel.
5. **Inverted.** Put everything back, then set Output Black 255 and Output White 0: every colour
   turns to its opposite. Transparent parts stay transparent.
6. **Threshold.** Put everything back, then Input Black and Input White both 120: each colour
   channel goes all the way on or off, a hard poster look.
7. **Out of range.** Type 300 in Input Black: it is refused with a sentence saying it runs from
   0 to 255, and the card keeps its old number. Type 20 in Gamma: refused, 0.1 to 10.
8. **Keyed.** Key Gamma at 1 and at 3 at a later frame. Scrub between: the middle lightens
   smoothly, with no jumps.
9. **Draft.** Press **Draft**: the picture is smaller, but the colours are the same.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- One set of levels for all three channels; there are no separate red, green and blue levels.
  Curves has those.
- It works on the 0 to 255 colour a drawing program shows, so a colour brighter than white from
  an effect above it is held at white first.

## What to answer

"works", or which step number did something else and what it did.
