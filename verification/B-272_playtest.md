# B-272: Split

Built on 2026-10-09 under your effects loop request, decided as D-393. **Split** (in
**Distort**) is our version of CycoreFX's CC Split: it tears the layer open along the line
between two points, the gap widest in the middle and closed at the ends, like an eye opening or
a crack. The formulas are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **Point A** (25, 50) and **Point B** (75, 50): per cent of the layer, the two ends of the tear.
- **Split** (50): pixels, how wide the gap opens at the middle. Each side is squeezed outward
  so nothing is lost; past half the tear's length to either side the picture is untouched.

Nothing grows. The gap is see-through, so whatever is underneath shows in it.

The check, `verification/D-393_split_table.md` (124 of 124), holds every pixel to numbers worked
out by a separate program before the code existed, and the graphics card's picture to within 1
level of the processor's. The pictures are in `verification/D-393 pictures/`. They are saved
over white, so the gap shows as white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: an eye-shaped white gap across the middle, widest in the
   centre, the buildings above pushed up and those below pushed down.
2. **Wide open.** `3_wide.png`, Split 150: the gap opened wide, the two halves squeezed up and
   down.
3. **Corner to corner.** `4_diagonal.png`, from (10, 90) to (90, 10), Split 40: a slanted tear.
4. **In the app.** Add Split over a second layer, key Split from 0 to 300 over a second and play:
   the layer tears open and the layer underneath shows through the gap.
5. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Split costs too little to read against the timing noise, and about 11.1 ms a
1080p layer on the processor (`verification/B-270_distort_timing_table.md`; provisional, the card
figures were within their own noise).

## Not built

- The shape of the gap (a sine, widest in the middle) and how far each side is squeezed are our
  own reading of CycoreFX's one-sentence descriptions; no tutorial with numbers was matched.
- There are no point handles on the viewer yet; set the points in Effect Controls.
- **CC Split 2** (each side of the tear opened by its own amount) is not built. It is the same
  rule with two split amounts, so it is your choice whether it becomes a second name over
  Split's engine, as Levels and Levels (Individual Controls) are, or a "Separate Sides" option
  on Split itself.
