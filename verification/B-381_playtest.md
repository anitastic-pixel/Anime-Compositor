# B-381: Difference Matte

Built on 2026-10-10 as D-501, under your /loop request: Difference Matte, in **Lines & Mattes**
(beside the other keyers), with After Effects' settings. It keys out every pixel of a layer that
matches a clean plate: another layer of the same shot taken without the subject.

- **View** (Final Output): Final Output shows the keyed layer; Matte Only shows what is kept as
  white and what is keyed as black.
- **Difference Layer** (none): the clean plate. Until one is chosen nothing is keyed.
- **If Layer Sizes Differ** (Center): a plate of another size is centred on the layer, or
  stretched to fit it.
- **Matching Tolerance** (15): how different from the plate a pixel may be, in per cent, and
  still be keyed out.
- **Matching Softness** (0): pixels a little more different than the tolerance fade in instead
  of staying whole. 0 keeps a hard edge.
- **Blur Before Difference** (0): blurs the layer and the plate before they are compared, so
  grain does not stop a match. The picture itself is never blurred.

Adobe publishes no formula and no starting values, so the rule and the settings when added are
our own, written down in document 21.

The check, `verification/D-501_difference_matte_table.md` (173 of 173), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-501 pictures/`.

## What to check

1. **The shot and the plate.** `1_shot.png`: a street with a magenta disc in front of it.
   `2_plate.png`: the same street without the disc.
2. **Keyed.** `3_keyed.png`, the plate as the Difference Layer, as added: only the disc is left;
   the whole street is gone (the page shows through).
3. **The matte.** `4_matte.png`, Matte Only: the disc white, everything else black.
4. **A change of light.** `5_lighter_plate.png` is the plate 6 levels lighter, as if the light
   changed between the takes. Against it, `6_lighter_tolerance_1.png` (tolerance 1) keys almost
   none of the street (only spots already at full brightness, which cannot get lighter), and `7_lighter_tolerance_5.png` (tolerance 5) keys all of it again. The disc
   is whole in both.
5. **As added.** With no Difference Layer chosen the picture is unchanged.
6. **In the app.** Put a clean plate under a shot of the same place with someone in it, switch
   the plate off, add Difference Matte (Lines & Mattes) to the shot and choose the plate as the
   Difference Layer. Raise Matching Tolerance until the background goes, Matching Softness to
   soften the edge, and Blur Before Difference if grain leaves specks. Try Matte Only. Keyframe
   the tolerance and play it.
7. **A missing plate.** Choose a plate, then delete that layer: the shot is left as it is, with
   a warning naming the missing layer.
8. **Out of range.** Type 300 in Matching Tolerance: it is refused with a sentence saying the
   range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: tolerance 15 when added, and how soft the edge looks as
Matching Softness rises.

## Speed

Measured on a quiet machine
(`verification/B-381_difference_matte_timing_table.md`). Three Difference Mattes on a 1920 by 1080 shot, card / processor,
milliseconds a frame played again: the shot after a moving Noise 12.1 / 40.9 ms a frame, against layer 4 40.6 / 72.3, softness 10, blurred at 2 47.3 / 145.0; about 9.5 ms a 1080p layer on the card unblurred and 11.7 blurred at 2, over the row's Target P1 (1 ms), the plate layer drawn on the processor first (D-189 path, as Aerial Haze and Gradient Wipe are, about 10.8 ms a layer for Aerial Haze); on the processor about 10.5 and 34.7 ms a layer.
