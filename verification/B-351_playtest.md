# B-351: Fast Blur (Legacy)

Built on 2026-10-11 as D-471, under your /loop request: **Fast Blur (Legacy)**, in **Obsolete**,
after After Effects' old Fast Blur. Old projects and tutorials use it; After Effects itself now
turns it into a Fast Box Blur, and so does ours, inside: it is Fast Box Blur with the old
control. Fast Box Blur itself is unchanged.

- **Blurriness** (0, 0 to 1000): how soft. It is Fast Box Blur at 0.37 times this radius, laid on
  3 times.
- **Blur Dimensions** (Horizontal and Vertical): or only across, or only down.
- **Repeat Edge Pixels** (Off): On keeps the edges solid instead of fading to see-through.

Adobe does not publish how Blurriness maps to Fast Box Blur; the 0.37 is Stu Maschwitz's match
(Blurriness 100 looks like radius 37), so it is not matched to After Effects pixel for pixel.

The check, `verification/D-471_fast_blur_legacy_table.md` (92 of 92), holds every pixel to numbers
worked out by a separate program before the code existed, holds the graphics card's picture to
within 1 level of the processor's, and checks that it draws exactly Fast Box Blur's picture. The
pictures are in `verification/D-471 pictures/`: the street (`town.png`, `1_before.png` without the
effect).

## What to check

1. **Soft.** `2_blurriness_20.png`: the street evenly softened.
2. **The same as Fast Box Blur.** `3_fast_box_radius_7_4.png`: Fast Box Blur at radius 7.4 with 3
   iterations; it should look exactly like `2_blurriness_20.png` (the same picture, byte for byte).
3. **Across only.** `4_horizontal_40.png`: the street smeared sideways only; the white road line
   stays sharp top and bottom.
4. **In the app.** Add Fast Blur (Legacy) (Blur & Sharpen, next to Fast Box Blur) to a picture; drag
   Blurriness; switch Blur Dimensions; turn on Repeat Edge Pixels and see the edges stay solid.
5. **Out of range.** Type 1001 in Blurriness: it is refused with a sentence giving the range.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-351_fast_blur_legacy_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Fast Blur (Legacy) at Blurriness 10 on each of three layers, played again,
15.4 ms a frame on the card against 12.8 for the Noise alone; on the processor
75.7 against 41.6: a busy machine cannot say whether the card is quicker than the processor for this effect.
