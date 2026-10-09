# B-249: Change Color

Built on 2026-10-09 under your effects loop request, decided as D-370. **Change Color** (in
**Color Correction**) is our version of After Effects' Change Color: the colours near one colour
you pick are turned round the colour wheel and made lighter or darker, stronger or greyer. It
shares Change to Color's colour wheel but is its own effect.

The settings, with the values they start at:

- **View** (Corrected Layer): Color Correction Mask shows which pixels change, white fully,
  black not at all.
- **Hue Transform** (0): degrees, -3600 to 3600, round the colour wheel.
- **Lightness Transform** and **Saturation Transform** (0 each): -100 to 100, toward black or
  white, toward grey or full colour.
- **Color To Change** (red).
- **Matching Tolerance** (15) and **Matching Softness** (0): 0 to 100, how near a colour must be
  to change fully, and how far beyond that the change fades out.
- **Match Colors** (Using Hue): Using RGB, Using Hue or Using Chroma.
- **Invert Color Correction Mask** (Off): On changes everything except the picked colour.

The check, `verification/D-370_change_color_table.md` (163 of 163), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-370 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: exactly the same as before.
2. **Recolour.** `3_red_walls_to_green.png`, Hue Transform 120, Softness 8: the red-brown
   houses turn green; the lit windows and blue houses stay as they were.
3. **The mask.** `4_mask_view.png`, View Color Correction Mask, Softness 8: white on the red
   houses, black elsewhere.
4. **Keep one colour.** `5_only_red_kept.png`, Saturation Transform -100, Softness 8, Invert
   On: everything goes grey but the red-brown houses.
5. **In the app.** Key Hue Transform from 0 to 360 over two seconds: the red houses cycle
   through every colour and come back.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Change Color costs under 1 ms a 1080p layer, about 2.5 ms on the processor
(`verification/B-247_colour_timing_table.md`).

## Not built

- The colours are compared on the picture's sRGB values; Adobe does not publish its method, so
  the rule is ours.
- A grey has no hue, so with Using Hue a grey is never near the colour.
- With Using Chroma the distance is the plain colour difference, not scaled to 0 to 100, and
  Softness is a distance beyond the tolerance, not a share of it as in Change to Color.
- No eyedropper; pick the colour in the colour box.
