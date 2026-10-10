# B-277: Color Grade, second part

Built on 2026-10-09 under your effects loop request, decided as D-398. **Color Grade** (in
**Color Correction**, B-276) gains the rest of Lumetri Color's main sections:

- **Curves**: a Master, Red, Green and Blue curve, drawn and typed as the Curves effect's are
  (pick the curve shown above the graph), and **Hue vs Saturation**, typed as points of a hue
  (0 to 359 degrees) and its saturation (100 as it was, 0 grey, 200 twice as strong).
- **Color Wheels**: Shadows, Midtones and Highlights, each a Wheel Hue, a Wheel Amount and a
  Lightness.
- **HSL Secondary**: a key that picks colours by hue (Key Hue with its Range and Softness),
  saturation and lightness (each a Low and a High, with one Key Softness), Invert Key, and
  Show Key (Mask shows the key as black and white). The picked colours then get their own
  Temperature, Tint, Contrast, Saturation, a wheel and a Lightness.

Everything new starts where it changes nothing, so a Color Grade from the first part looks the
same as before. Every number can be keyed. These run after the first part's sections and before
the vignette.

The check, `verification/D-398_color_grade_2_table.md` (235 of 235 checks pass), holds every pixel to
numbers worked out by a separate program before the code existed, and the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-398 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **S-curve.** `2_s_curve.png`: a Master curve bent into an S. Darks darker, lights lighter,
   more contrast; pure black and white stay.
2. **Hue curve.** `3_hue_curve.png`: Hue vs Saturation with the greens at 200 and the blues at 0.
   Greens stronger, the blue sky grey, reds as they were.
3. **Wheels.** `4_wheels.png`: a teal Shadows wheel and an orange Highlights wheel. Dark parts
   teal, light parts warmer.
4. **Colour pop.** `5_colour_pop.png`: the reds keyed, Invert Key on, Secondary Saturation 0.
   Everything grey except the reds.
5. **The key.** `6_key_mask.png`: the same with Show Key on Mask. Black and white only: white
   where the correction applies (everything but the reds), black on the reds.
6. **As added.** Add Color Grade and change nothing: the picture should not change at all.
7. **In the app.** Pick the Red curve above the graph and drag its middle up: the picture warms.
   Type `0 100, 120 200, 240 100` in Hue vs Saturation points and press Enter: greens stronger.
   Key Hue from 0 to 120 over a second with Range 20 and Secondary Saturation 0: the grey moves
   from the reds to the greens.
8. **Saved and opened again.** Change curves, wheels and the key, save, close and open again.
   You should get the same settings and the same picture. An older project with a Color Grade
   should open looking as it did.

**Timing.** Provisional, the machine was busy with another lane's tests on the same graphics card: the card's figures could not be read in that run; on the processor about 20 ms a layer for the curves, hue curve and wheels and 13 ms for the secondary (`verification/B-277_color_grade_2_timing_table.md`). To be measured again on a quiet machine.

## Not built

- **The key's Denoise and Blur**, which smooth the key's edges in Lumetri.
- **The other hue curves**: Hue vs Hue, Hue vs Luma, Luma vs Saturation and Saturation vs
  Saturation.
- **The secondary's three wheels.** Lumetri's secondary can use three wheels; ours has one.
- **The eyedropper** for picking the key's colour from the picture; type the hue instead.
- **Dragging a point on a wheel.** Lumetri's wheels are a disc you drag; ours are a hue and an
  amount, which set the same thing.
- **The method is ours.** Adobe does not publish how Lumetri works. The curves are the Curves
  effect's, the wheels Color Balance's, the hue curve Vibrance's at each hue's saturation; the
  key's shape is our own.
