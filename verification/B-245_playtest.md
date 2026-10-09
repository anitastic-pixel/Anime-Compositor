# B-245: Color Neutralizer

Built on 2026-10-08 under your effects loop request, decided as D-366. **Color Neutralizer** (in
**Color Correction**) is our version of CycoreFX's CC Color Neutralizer: you pick the colour that
should have been grey in the shadows, in the midtones and in the highlights, and each is pulled
to grey at its own lightness, taking a colour cast out.

The settings, with the values they start at:

- **Shadows, Midtones and Highlights Unbalance** (black, middle grey, white): the colour that
  should be grey there. Already grey changes nothing.
- **Shadows, Midtones and Highlights** (0, 0, 0 each): red, green and blue levels to add there,
  -255 to 255, to push further or tint.
- **Pinning** (0): 0 to 100, how much pure black and white are held where they are.
- **Black Point** (0) and **White Point** (255): where the shadows and highlights sit.

The check, `verification/D-366_color_neutralizer_table.md` (123 of 123), holds every pixel to
numbers worked out by a separate program before the code existed, and the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-366 pictures/`.

## What to check

**`warm.png`** is the street with a warm cast laid on it (more red, less blue).
**`1_before_warm.png`** is it with no effect: the grey road is brown.

1. **Cast removed.** `2_neutralized.png`: the brown road's colour (#54442a) picked as the
   shadows' and midtones' unbalance and the cream road markings' (#ffffe2) as the highlights'.
   The road should be grey again and the whole street close to `town.png`.
2. **Tinting with the numbers.** `3_numbers_cool_shadows_warm_highlights.png`: the plain street
   with blue 60 added to the shadows and red 40, green 20 to the highlights: a blue road, warm
   lights.
3. **In the app.** Put it on footage with a cast, set the Midtones Unbalance to the colour of
   something that should be grey: the cast goes.
4. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Color Neutralizer costs under 1 ms a 1080p layer, about 4 ms on the
processor (`verification/B-244_colour_timing_table.md`).

## Not built

- No eyedropper that averages an area of the viewer; type or pick the colour in the colour box.
- CycoreFX's View Correction graphs are not built.
- In CycoreFX picking a colour sets the numbers; here the colour and the numbers add together.
