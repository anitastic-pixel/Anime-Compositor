# B-106: Rain, by hand

Built on 2026-09-26 against D-163, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the thirtieth of the thirty.

The generated halves are `verification/B-106_rain_table.md`, which renders every FX-RAIN case
against the numbers written before the code, and `verification/B-12b_state_fields_table.md`,
which checks the Rain card sends every setting the command reads. This sheet covers what the
tables cannot: how it looks and feels in the window. The picture in `verification/B-106a
proposal/` shows what to expect.

## Before you start

Open a project at least 1280 by 720 with a dark blue full-frame solid, and a character drawing
above it. Select the solid and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Rain** in **Add effect…** on the solid. It goes to the end of the stack,
   and the card shows **Colour** a pale blue, **Density** 30, **Spacing** 24, **Length** 20,
   **Width** 1, **Direction** 170, **Speed** 30, **Seed** 0 and **Opacity** 60. Thin pale
   streaks, scattered, leaning a little, cover the whole frame.
2. **Falling.** Play: the streaks slide down and a little to the right, steadily, new ones
   coming in at the top as others leave at the bottom.
3. **Straight down.** Direction 180: the streaks stand upright and fall straight down.
   Direction 200: they lean and fall the other way.
4. **Heavier rain.** Density 70, Spacing 12: many more streaks, closer together. Density 0: no
   rain at all.
5. **Longer and thicker.** Length 60, Width 3: long, heavier streaks. Width 0: faint hairlines.
6. **Speed.** Speed 5: a slow drizzle. Speed 0: the streaks hang still.
7. **Seed and colour.** Seed 5: a different pattern of streaks. Seed 5.7 gives the same as 5.
   Colour white, Opacity 100: bright white streaks.
8. **Over a picture.** Put the effect on the character drawing instead of the solid: it rains
   only on the drawing, nowhere outside it.
9. **Out of range.** Type 1 in Spacing: it is refused with a sentence saying it runs from 2 to
   1000, and the card keeps its old number.
10. **Keyed.** Key Opacity at 0 and at 60 two seconds later. Play: the rain fades in.
11. **Draft preview.** Switch to a half-size draft: the streaks are the same size, as far
    apart, and fall as fast on screen as at full size.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there, and the same frame shows the same streaks.

## Known limits, on purpose

- **One layer of rain.** Straight streaks of one length and width, all at one depth, with no
  splashes, no drops hitting the drawing and no gusts. For depth, stack two Rain effects with
  different seeds, sizes and speeds.
- **Overlaps show the stronger streak**, not the two added.
- **Only inside the covering.** It rains only where the layer shows, so a whole-frame rain
  needs a full-frame solid.
- **Tiny layers see few drops.** A layer smaller than the spacing can go several frames with no
  streak at all.
- It is modelled on After Effects' CC Rainfall in spirit, not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
