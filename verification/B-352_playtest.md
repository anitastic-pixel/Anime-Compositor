# B-352: Gaussian Blur (Legacy)

Built on 2026-10-11 as D-472, under your /loop request: **Gaussian Blur (Legacy)**, in
**Obsolete**, after After Effects' old Gaussian Blur. Old projects and tutorials use it; After
Effects itself now turns it into a Gaussian Blur, and so does ours, inside: it is Gaussian Blur
with the old controls. Gaussian Blur itself is unchanged.

- **Blurriness** (0, 0 to 500): how soft, the same as Gaussian Blur's Blurriness.
- **Blur Dimensions** (Horizontal and Vertical): or only across, or only down.

It has no Repeat Edge Pixels, as the old effect had none: the edges always fade to see-through.
Adobe does not publish the old effect's exact softness, so it is not matched to After Effects
pixel for pixel.

The check, `verification/D-472_gaussian_blur_legacy_table.md` (82 of 82), holds every pixel to
numbers worked out by a separate program before the code existed, holds the graphics card's
picture to within 1 level of the processor's, and checks that it draws exactly Gaussian Blur's
picture. The pictures are in `verification/D-472 pictures/`: the street (`town.png`,
`1_before.png` without the effect).

## What to check

1. **Soft.** `2_blurriness_20.png`: the street evenly softened.
2. **The same as Gaussian Blur.** `3_gaussian_blur_20.png`: Gaussian Blur at Blurriness 20; it
   should look exactly like `2_blurriness_20.png` (the same picture, byte for byte).
3. **Across only.** `4_horizontal_40.png`: the street smeared sideways only; the white road line
   stays sharp top and bottom.
4. **In the app.** Add Gaussian Blur (Legacy) (Blur & Sharpen, next to Gaussian Blur) to a
   picture; drag Blurriness; switch Blur Dimensions; see the edges fade.
5. **Out of range.** Type 501 in Blurriness: it is refused with a sentence giving the range.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-352_gaussian_blur_legacy_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a
moving Noise and a Gaussian Blur (Legacy) at Blurriness 10 on each of three layers, played again,
63.5 ms a frame on the card against 29.7 for the Noise alone; on the processor
251.9 against 124.1: a busy machine cannot say whether the card is quicker than the processor for this effect.
