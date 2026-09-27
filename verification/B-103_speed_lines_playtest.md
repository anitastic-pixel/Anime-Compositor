# B-103: Speed Lines, by hand

Built on 2026-09-26 against D-160, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twenty-seventh of the thirty.

The generated halves are `verification/B-103_speed_lines_table.md`, which renders every
FX-SPEED case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Speed Lines card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-103a proposal/` shows what to expect.

## Before you start

Open a project at least 1280 by 720 with a white full-frame solid on top and a character
drawing below it. Select the solid and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Speed Lines** in **Add effect…** on the solid. It goes to the end of
   the stack, and the card shows **Centre** 50, 50, **Colour** black, **Count** 120,
   **Thickness** 1.5, **Inner** 150, **Inner jitter** 40, **Angle jitter** 50, **Seed** 0,
   **Hold** 2 and **Opacity** 100. Thin black lines rush in from the edges toward the middle,
   stopping short of it, the middle left white: manga focus lines.
2. **Redrawn on twos.** Play: the lines jump to a new set every two frames, as hand-drawn focus
   lines do. Hold 1: every frame. Hold 100: they stay still.
3. **Seed.** Seed 5: a different set of lines. Seed 5.7 gives the same as 5.
4. **Count and thickness.** Count 40, Thickness 6: fewer, wider wedges.
5. **Inner.** Inner 400: the clear middle grows. Inner 0: the lines reach the centre.
6. **Jitter.** Inner jitter 0 and Angle jitter 0: the lines are perfectly even and all stop at
   the same ring, which looks mechanical; raising them again roughens it.
7. **Centre.** Centre 30, 40: the lines rush toward that point instead.
8. **Colour and opacity.** Colour a blue, Opacity 50: pale blue lines.
9. **Over a picture.** Put the effect on the character drawing instead of the solid: the lines
   are drawn only on the drawing, nowhere outside it.
10. **Out of range.** Type 3 in Count: it is refused with a sentence saying it runs from 4 to
    1000, and the card keeps its old number.
11. **Draft preview.** Switch to a half-size draft: the clear middle is the same size on screen
    as at full size.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    is still there, and the same frame shows the same lines.

## Known limits, on purpose

- **Starts too far out for small layers.** Inner starts at 150 pixels, so on a small layer
  nothing shows until Inner is lowered.
- **Straight wedges only**: no tapered tips, curves or brush texture.
- **Thickness 0 still draws faint hairlines**, from the half-pixel softening of each edge.
- With Inner 0, the few pixels right at the centre turn grey where every line's edge meets.
- It is modelled on the focus lines of manga and anime, not on any one After Effects effect.

## What to answer

"works", or which step number did something else and what it did.
