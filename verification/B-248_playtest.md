# B-248: Toner

Built on 2026-10-09 under your effects loop request, decided as D-369. **Toner** (in **Color
Correction**) is our version of CycoreFX's CC Toner: the picture is coloured by its lightness
with two, three or five tones, from shadows to highlights, for duotones and sepia prints. It
shares Gradient Map's lightness but is its own effect.

The settings, with the values they start at:

- **Tones** (Tritone): Duotone uses Shadows and Highlights; Tritone adds Midtones; Pentone
  adds Darktones and Brights too. Tones not used are kept.
- **Highlights** (white), **Brights** (#e0cfb0), **Midtones** (#8c7355), **Darktones**
  (#46382a), **Shadows** (black): a warm sepia of our own, as CycoreFX's starting colours are
  not published.

The check, `verification/D-369_toner_table.md` (105 of 105), holds every pixel to numbers worked
out by a separate program before the code existed, and the graphics card's picture to within 1
level of the processor's. The pictures are in `verification/D-369 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **Sepia.** `2_sepia_tritone.png`, as added: an old brown photograph.
2. **Black and white.** `3_duotone_black_white.png`, Duotone: the street in plain greys.
3. **Two colours.** `4_duotone_purple_gold.png`, Duotone, Shadows #2b0a3d, Highlights #ffd166:
   a poster-like purple and gold street.
4. **Five colours.** `5_pentone_purple_orange.png`, Pentone, near-black purple through violet
   and orange to pale yellow: a sunset-coloured street.
5. **In the app.** Switch Tones between Duotone, Tritone and Pentone and change a colour: the
   picture follows at once.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Toner costs about 1 ms a 1080p layer, about 3 ms on the processor
(`verification/B-247_colour_timing_table.md`).

## Not built

- No eyedroppers; pick the colours in the colour boxes.
- CycoreFX's Blend w. Original is the Mix every effect has.
