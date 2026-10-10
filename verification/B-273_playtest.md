# B-273: Split 2

Built on 2026-10-09, decided as D-394 by your choice "Second name, one engine (Recommended)".
**Split 2** (in **Distort**) is our version of CycoreFX's CC Split 2: Split's tear with each
side opened by its own amount. It is a second effect name over Split's engine, as Levels
(Individual Controls) is over Levels; Split itself is unchanged and old projects draw exactly as
before.

The settings, with the values they start at:

- **Point A** (25, 50) and **Point B** (75, 50): per cent of the layer, the two ends of the tear.
- **Split 1** (50): pixels, how far the side on your **left** opens, as you walk from A to B.
  For the points as added (A on the left, B on the right) that is the upper side.
- **Split 2** (50): pixels, how far the side on your **right** opens; for the points as added,
  the lower side.

With both at the same amount it is Split's own picture, pixel for pixel. Set one to 0 and that
side stays exactly where it was.

The check, `verification/D-394_split_2_table.md` (139 of 139), holds every pixel to numbers
worked out by a separate program before the code existed, the graphics card's picture to within
1 level of the processor's (it came out identical on every test file), and redraws Split's own
three pictures from B-272 to confirm they are byte for byte unchanged. The pictures are in
`verification/D-394 pictures/`. They are saved over white, so the gap shows as white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`, both sides 50: the same eye-shaped gap as Split's
   `verification/D-393 pictures/2_as_added.png`.
2. **One side only.** `3_upper_only.png`, Split 1 150, Split 2 0: only the upper half is pushed
   up into a dome; everything below the line, the street and the lower buildings, is untouched.
3. **Uneven.** `4_lower_wide.png`, Split 1 20, Split 2 200: the gap barely rises above the line
   and bulges far downward.
4. **Corner to corner.** `5_diagonal.png`, from (10, 90) to (90, 10), Split 1 0, Split 2 60:
   walking up and to the right, your right is the lower-right side, so only that side opens and
   the sky above the line is untouched.
5. **Old projects.** Open any project saved with Split before today: it looks exactly as it did.
6. **In the app.** Add Split 2 over a second layer, key Split 2 from 0 to 300 over a second with
   Split 1 at 0 and play: only the lower side opens and the layer underneath shows through.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Split 2 costs about 1.3 to 1.5 ms a 1080p layer on the graphics card and about
10.1 ms on the processor (`verification/B-273_split_2_timing_table.md`, a quiet machine).

## Not built

- Which side is "1" is our choice (your left walking from A to B); CycoreFX publishes no
  formula or diagram for it. Swap the points to swap the sides.
- There are no point handles on the viewer yet; set the points in Effect Controls.
- No tutorial with numbers was found to reproduce.
