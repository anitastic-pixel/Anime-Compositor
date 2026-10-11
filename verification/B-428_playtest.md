# B-428: Spill Suppressor

Built on 2026-10-11 as D-548. After Effects' old **Spill Suppressor** (Keying, now Obsolete) is
back under its own name, as a second name over our **Screen Key**'s despill: the same maths,
with Screen Key's matte turned off so nothing is keyed out. It takes the screen colour's
spill off the edges of a subject already keyed.

- **Color To Suppress** (blue as added): pick the screen's colour. Only its strongest channel
  matters: green for a green screen, blue for a blue one.
- **Suppression** (100 as added): how much of the spill comes off, 0 to 100 per cent. Can be
  keyed.

At 100, every pixel's green (for a green screen) is held no higher than the average of its red
and blue; red, blue and every covering are never touched.

Not built: After Effects' Color Accuracy (Faster or Better). There is one careful rule for
every colour, so there is nothing to choose.

The check, `verification/D-548_spill_suppressor_table.md` (169 of 169 checks), holds every pixel to numbers
worked out by a separate program before the code existed, runs Screen Key's 24 earlier checks
again unchanged, shows Spill Suppressor drawing exactly what Screen Key draws with the same
despill, and holds the graphics card's picture to within 1 level of the processor's. The
pictures are in `verification/D-548 pictures/`.

## What to check

1. **Before.** `1_before.png`: the town with no effect.
2. **As added.** `2_as_added.png`: blue suppressed: the blue sky and the blue buildings lose
   their blue and turn a grey green; the red, green and tan buildings and the road unchanged.
3. **Green screen.** `3_green.png`: green suppressed: the green buildings turn a brownish grey
   and the tan ones a little warmer, their lit windows peach; the sky, the blue and red
   buildings and the road unchanged.
4. **Half.** `4_green_50.png`: the same at 50, half way between 1 and 3.
5. **Red.** `5_red.png`: red suppressed: the red buildings turn a dark olive and the tan ones a
   pale green; the sky and the blue and green buildings unchanged.
6. **In the app.** Key a green-screen shot with Color Key or Extract, then add Spill Suppressor
   (Lines & Mattes) and pick the screen's green with the colour picker: the green fringe on the
   edges turns neutral. Lower Suppression to bring some back.
7. **Out of range.** Type 101 in Suppression: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: a colour with no strongest channel (a grey) counts as
green, and the spill is measured against the plain average of the other two channels.

## Speed

On the reference shot with three noisy layers it takes the graphics card from 23.4 to 37.2 milliseconds a frame as added, and the processor from 42.6 to 70.9; these are PROVISIONAL, measured while other builds were running (`verification/B-428_spill_suppressor_timing_table.md`).
