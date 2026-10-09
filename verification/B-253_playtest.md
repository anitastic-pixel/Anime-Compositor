# B-253: Color Balance (HLS)

Built on 2026-10-09 under your effects loop request, decided as D-374. **Color Balance (HLS)**
(in **Color Correction**) is our version of After Effects' Color Balance (HLS): every colour of
the layer is turned round the colour wheel, and made lighter or darker, stronger or greyer, by
the same amount everywhere. It is a separate effect from Hue/Saturation, whose Lightness pushes
toward black or white by a share; this one adds its amounts straight on.

The settings, with the values they start at:

- **Hue** (0): degrees, -3600 to 3600, round the colour wheel.
- **Lightness** (0): -100 to 100, added to every colour's lightness.
- **Saturation** (0): -100 to 100, added to every colour's strength. A grey stays grey.

The check, `verification/D-374_color_balance_hls_table.md` (110 of 110), holds every pixel to
numbers worked out by a separate program before the code existed, and the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-374 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: exactly the same as before.
2. **Turn the colours.** `3_hue_120.png`, Hue 120: the red-brown houses turn green, the green
   ones blue, the blue ones pink, and the sky pink.
3. **Grey.** `4_saturation_-100.png`, Saturation -100: the whole street in greys.
4. **Darker.** `5_lightness_-30.png`, Lightness -30: every pixel darker; white goes grey.
5. **In the app.** Key Hue from 0 to 360 over two seconds: the street cycles through every
   colour and comes back.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

**Timing.** On the reference shot, three 1080p layers with a moving Noise, played again at Full: 11.5 ms a frame without, 15.1 with Color Balance (HLS) on each, drawn by the graphics card, about 1 ms a layer (`verification/B-253_colour_timing_table.md`).

## Not built

- Adobe does not publish its method, so the rule is ours (document 21); worked on the
  picture's sRGB values.
