# B-82: Channel Mixer, by hand

Built on 2026-09-26 against D-139, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the sixth of the thirty.

The generated halves are `verification/B-82_channel_mixer_table.md`, which renders every
FX-MIXER case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Channel Mixer card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-82a proposal/` shows what to expect.

## Before you start

Open a project with a coloured drawn layer (a cel with red, green and blue in it, and some empty
space around it), select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Channel Mixer** in **Add effect…**. It goes to the end of the stack. The
   card shows three rows of four numbers, **Red** 100, 0, 0, 0, **Green** 0, 100, 0, 0 and
   **Blue** 0, 0, 100, 0, and **Monochrome** Off. The drawing does not change at all.
2. **Swap red and blue.** Set Red to 0, 0, 100, 0 and Blue to 100, 0, 0, 0. Red hair turns blue
   and blue sky turns red; greens stay as they were.
3. **Grey by your own weights.** Set Monochrome to On and Red to 30, 59, 11, 0. The drawing turns
   grey, lit as the eye sees it. Try Red 0, 100, 0, 0: the grey now shows only how much green was
   in each colour.
4. **A constant.** Monochrome Off, Green 0, 100, 0, 20. Everything leans green, even black lines,
   which turn dark green.
5. **Negative.** Red -100, 0, 0, 100: the red channel is inverted, so reds go dark and cyans pick
   up red.
6. **Soft edges and empty space.** Soft edges stay soft, and the empty parts stay empty, whatever
   the numbers.
7. **Out of range.** Type 201 in any of Red's four fields: it is refused with a sentence saying it
   runs from -200 to 200, and the card keeps its old numbers.
8. **Keyed.** Key Red at 100, 0, 0, 0 and at 0, 0, 100, 0 at a later frame. Play: reds fade and
   blues come into the red channel, all four numbers moving together.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- With Monochrome On, **only the Red row is read**; Green and Blue are kept but do nothing until
  it is turned Off. This is how After Effects' Channel Mixer reads its monochrome switch, and it
  may surprise: if a grey looks wrong, check the Red row, not the others.
- The mix is worked on the stored (sRGB) values, in per cent of the channel's whole scale, and
  each channel is clipped to black and white after mixing.
- A row is keyed as one value of four numbers: its four numbers always move together.
- It is modelled on After Effects' Channel Mixer and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did. And whether Monochrome should
read the Red row (as now) or have its own row of weights.
