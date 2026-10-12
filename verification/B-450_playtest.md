# B-450: Color Grade, third part

Built on 2026-10-12 under your effects loop request, decided as D-571. **Color Grade** (in
**Color Correction**, B-276 and B-277) gains the last of Lumetri Color's missing pieces:

- **Four more curves**, typed as points like Hue vs Saturation:
  - **Hue vs Hue**: a hue (0 to 359) and how far to turn it (-180 to 180 degrees). Greens can
    become cyan without touching reds.
  - **Hue vs Luma**: a hue and how much lighter or darker (-100 to 100). Darken the sky, lighten
    the walls.
  - **Luma vs Saturation**: a brightness (0 to 100) and a saturation (100 as it was, 0 grey, 200
    twice as strong). Grey out the shadows, for example.
  - **Saturation vs Saturation**: a saturation (0 to 100) and its new saturation. Strengthen dull colours, calm
    loud ones.
- **HSL Secondary, three wheels**: Secondary Wheels set to **Three** adds a Shadow and a
  Highlight wheel (each a hue, an amount and a lightness) beside the old wheel, which becomes the
  midtones'. Left on **One**, the new wheels do nothing.
- **Key Denoise and Key Blur**: tidy the key before it is used. Denoise rounds off specks and
  sharp corners; Blur softens its edge. Show Key on Mask shows the tidied key.

Everything new starts where it changes nothing, so a Color Grade from the first two parts looks
the same as before. Every number can be keyed.

The check, `verification/D-571_color_grade_3_table.md` (195 of 195 checks pass), holds every pixel
to numbers worked out by a separate program before the code existed, and the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-571 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **Hue vs Hue.** `2_hue_vs_hue.png`: greens turned toward cyan-green and blues toward purple;
   the red and orange buildings as they were.
2. **Hue vs Luma.** `3_hue_vs_luma.png`: the blue sky and blue buildings darker, the warm walls
   lighter.
3. **Luma vs Saturation.** `4_luma_vs_saturation.png`: the dark road and window holes grey, the
   bright colours stronger.
4. **Saturation vs Saturation.** `5_saturation_vs_saturation.png`: the pale colours stronger, the strong ones
   calmer.
5. **Three wheels.** `6_three_wheels.png`: every colour keyed, Secondary Wheels on Three, a teal
   shadow wheel and an orange highlight wheel: the dark road teal, the light sky warmer.
6. **The key, tidied.** `7_key_mask.png` is the reds keyed, shown as the mask: white buildings
   with square windows. `8_key_denoised.png` is the same with Key Denoise 30: the corners and
   windows rounded. `9_key_blurred.png` is Key Blur 30: the edges soft.
7. **As added.** Add Color Grade and change nothing: the picture should not change at all.
8. **In the app.** Type `0 0, 120 40, 240 0` in Hue vs Hue points and press Enter: the greens
   shift toward cyan. Key the reds (Key Hue 0, Range 20), Show Key on Mask, then raise Key Blur:
   the mask's edges soften. Set Secondary Wheels to Three and move the Secondary Shadow Wheel
   Amount: only the dark keyed parts change.
9. **Saved and opened again.** Change the new curves, the wheels and the key's denoise and blur,
   save, close and open again: the same settings and picture. An older project with a Color
   Grade should open looking as it did.

**Timing.** On the reference shot with three noisy layers, the four curves and three wheels cost about 10 ms a layer on the graphics card and about 50 on the processor. A key with Denoise or Blur is worked out by the processor, and with the card on it came out slower than with the card off (737 against 349 ms a frame); this is to be measured again. These are PROVISIONAL, measured while other builds were running (`verification/B-450_color_grade_3_timing_table.md`).

## Not built

- **Highlight recovery past white.** Our grade works on colours held between black and white,
  so there is nothing past white to bring back, and Adobe does not say how Lumetri does it.
  Lower Highlights or Whites instead.
- **The eyedropper** for picking the key's colour from the picture; type the hue instead.
- **Dragging curve points or wheels.** The four new curves are typed as points; the wheels are a
  hue and an amount.
- **On the graphics card, a tidied key is worked out by the processor.** A grade with Key
  Denoise or Key Blur above 0 is drawn by the processor; the picture is the same, only slower.
- **The method is ours.** Adobe does not publish how Lumetri works. Hue vs Hue keeps each
  colour's lightness and saturation; the saturation curves are Vibrance's rule; the three wheels
  are Color Balance's; Denoise is the Median effect's and Blur the Gaussian Blur's.
