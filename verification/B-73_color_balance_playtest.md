# B-73: Color Balance, by hand

Built on 2026-09-26 against D-130, which you accepted the same day as the eighth of the second
batch of ten ("go ahead with the ten").

The generated halves are `verification/B-73_color_balance_table.md`, 76 of 76, which renders
every FX-BALANCE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Color Balance card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-73a proposal/` shows what to expect.

## Before you start

Open a project with a coloured drawn layer, a character or a background with light and dark
parts, select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Color Balance** in **Add effect…**. It goes to the end of the stack,
   and the card shows **Shadows**, **Midtones** and **Highlights**, each with R, G and B boxes,
   all nine at 0. The drawing does not change.
2. **Cool shadows, warm lights.** Shadows B 40 and Highlights R 30: the dark parts turn bluer and
   the light parts warmer, and the middle tones barely move.
3. **Midtones.** Midtones G 100: the middle tones turn green; the darkest and lightest parts
   hardly change.
4. **Held at the ends.** Highlights B 100 on a white part: it stays white, as its blue is already
   full. Shadows R, G and B all -100 on black: it stays black.
5. **Edges.** Soft line edges stay soft; empty parts stay empty.
6. **Out of range.** Type 101 in Shadows R: it is refused with a sentence saying it runs from
   -100 to 100, and the card keeps its old number.
7. **Keyed.** Key Shadows at 0, 0, 0 and at 100, 0, 0 at a later frame. Play: the shadows redden
   over time. The three boxes of one tone share one key.
8. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- The three tones are split by brightness alone, and the split is fixed: half brightness is
  the middle. There is no setting for where the shadows end.
- There is no "keep brightness" switch, so pushing a colour up also lightens.
- A tone has no colour picker, only its three numbers.
- It is modelled on After Effects' Color Balance and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
