# B-40: Bloom, by hand

Built on 2026-09-25 against D-96, which you accepted the same day as the sixth of the batch of
seven ("knowing the expectation, let's do a batch of 1 thru 7 in your recommended order").

The generated halves are `verification/B-40_bloom_table.md`, 106 of 106, which renders every
FX-BLOOM case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, 91 of 91, which checks the Bloom card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-40a proposal/` shows what to expect.

## Before you start

Use glow's flame again, `verification/B-33c_glow_drawing.png`. Its light pink core is at 100 %
and its light purple middle at 90 %, so at threshold 80 both bloom and nothing else does; at 95
only the core does. Import it, make a layer from it and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Bloom** in **Add effect…**. It goes to the bottom of the stack, and the
   card shows **Threshold** 80, **Radius** 20, **Intensity** 1, **Streak Length** 60, **Streak
   Angle** 0 and **Streaks** None. The core and the middle light up and a soft light spreads
   round them, as in the picture's third panel.
2. **Against glow.** Add a **Glow** with the same threshold and radius, and switch each off in
   turn. The bloom keeps more of its light close to the flame and fades sooner; the glow is a
   more even haze.
3. **Threshold.** Set Threshold to 95: only the core blooms. Set it to 100: the core still does,
   as its brightest channel is 255. Set it to 50: the dark purple outer flame blooms too.
4. **Streaks.** Set Threshold back to 95 and **Streaks** to **Cross**: beams of light run
   straight up and down and left and right from the core, as in the fourth panel. **Star** adds
   the diagonals, each beam a little fainter.
5. **Angle and length.** With Star, set Streak Angle to 20: the star turns clockwise. Set Streak
   Length to 120: the beams reach twice as far. Set Streaks to None: the beams go, and the length
   and angle no longer change anything.
6. **Past the edge.** Move the layer so the flame is near the frame's edge: the light still
   spreads past the drawing's own edge.
7. **Out of range.** Type 600 in Radius: it is refused with a sentence saying the radius runs
   from 0 to 500, and the card keeps its old number. Type 11 in Intensity: refused, 0 to 10.
8. **Keyed.** Key Intensity at 0 and at 2 at a later frame, and Streak Angle at 0 and 90. Scrub
   between: the light grows smoothly and the star turns, with no jumps.
9. **Draft.** Press **Draft**: the picture is smaller, but the halo and the beams look the same
   size against the flame.
10. **Undo and saved.** Ctrl+Z steps back each change, the Streaks word included. Save, close and
    open again: every setting and key is still there.

## Known limits, on purpose

- A beam is as wide as the bright part it comes from, so the whole middle of the flame at
  threshold 80 gives broad bands rather than thin rays. Raise the threshold for thin beams.
- The light adds up and is not cut off at white, as glow's Add; there is no Screen.
- A large radius, or a long star, on a large layer is slow: each pixel takes each beam's samples.
  P-17, the bug and performance pass after the batch, measures it.

## What to answer

"works", or which step number did something else and what it did.
